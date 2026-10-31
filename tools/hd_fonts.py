#!/usr/bin/env python3
"""Redraw the game's pictures of text from vector fonts, as indexed texture
replacements (docs/MODDING.md, "Indexed replacements").

Reads only what `pe2 --extract textures,files` writes under the data folder's
dumps: each picture's map (dumps/textures-glyphs, its glyphs' cells and
text), its sheets' indices (dumps/textures-indexed) and, for the title, its
palette (dumps/files). Writes `<sheet>-<4|8>bit.idx.png` and `.toml` into
the output folder, for a mod's `textures/`. Fonts are yours to choose (the
ones used so far are free: DejaVu, Courier Prime).

    python tools/hd_fonts.py ui      DUMPS FONT.ttf OUT [--weight -0.25]
    python tools/hd_fonts.py loading DUMPS FONT.ttf OUT [--weight 0.15]
    python tools/hd_fonts.py title   DUMPS REGULAR.ttf BOLD.ttf OUT
                                     [--serif SERIF.ttf] [--logo LOGO.svg]
                                     [--text "New Game=Novo Jogo"]... [--over DIR]

Each sheet is first enlarged smoothly (its indices' greys or colours
Lanczos-enlarged, each finer texel taking, of its own texel's index and its
8 neighbours', the nearest), so what is not redrawn (icons not known,
pictures) stays as it was, only smoother; then each glyph is drawn into
its cell from the font, in the indices the game's glyphs use:

- ui:      5 the page, 10 the outline ring, 15 the fill, those between
           the edges; letters of a face share one scale (its capital H's)
           and stand on the baseline their record gives;
- loading: 0 clear, 1 the ring, 15 the fill, those between the edges;
- title:   each row of text fitted to the original's ink, its coverage
           turned into the row's own shades (by brightness); a logo, from
           an SVG's paths (M, L, C, z) left of x = 320 in its units (the
           Squaresoft wordmark's SQUARE), in the logo's own colours;
           --text draws a row in another text (a translation), at the
           original's scale, on its baseline, centred where it was;
           --over starts from a mod's replacements of the page (redrawing
           only the rows given another text).

Needs numpy, pillow and scipy.
"""

import argparse
import glob
import os
import re
import sys
import tomllib

import numpy as np
from PIL import Image, ImageDraw, ImageFont
from scipy import ndimage

SS = 4  # supersampling for coverage


# ---- maps and sheets ---------------------------------------------------------------


def find_maps(dumps, kind):
    """Every map of `kind` (the loading font has two sheets)."""
    maps = []
    for path in sorted(glob.glob(os.path.join(dumps, 'textures-glyphs', '*.toml'))):
        with open(path, 'rb') as f:
            m = tomllib.load(f)
        if m.get('kind') == kind:
            maps.append(m)
    if not maps:
        sys.exit(f'no map of kind {kind!r} in {dumps}/textures-glyphs (extract with a current pe2 first)')
    return maps


def find_map(dumps, kind):
    return find_maps(dumps, kind)[0]


def read_sheets(dumps, m):
    bits = m['depth']
    parts = [np.array(Image.open(os.path.join(dumps, 'textures-indexed', f'{s}-{bits}bit.idx.png')))
             for s in m['sheets']]
    return np.hstack(parts).astype(np.int64)


def write_sheets(out_dir, m, page, k, note):
    os.makedirs(out_dir, exist_ok=True)
    bits = m['depth']
    width = 256 if bits == 4 else 128
    for n, sheet in enumerate(m['sheets']):
        part = page[:, n * width * k:(n + 1) * width * k]
        rows = part.shape[0] // k
        Image.fromarray(part.astype(np.uint8)).save(os.path.join(out_dir, f'{sheet}-{bits}bit.idx.png'))
        with open(os.path.join(out_dir, f'{sheet}-{bits}bit.idx.toml'), 'w', encoding='utf-8') as f:
            f.write('format = "pe2pc-indexed-texture"\nversion = 1\n'
                    f'sheet = "{sheet}"\ndepth = {bits}\nnative_width = {width}\nnative_height = {rows}\n'
                    f'width = {width * k}\nheight = {rows * k}\nlayout = "sheet-rows"\n'
                    f'depth_source = "{note}"\n')
        print(os.path.join(out_dir, f'{sheet}-{bits}bit.idx.png'))


def smooth(idx, k, colour):
    """`idx` k times finer: `colour` (index -> RGBA or grey) Lanczos-enlarged,
    each finer texel the nearest of its texel's index and its neighbours'."""
    h, w = idx.shape
    c = np.atleast_3d(colour[idx]).astype(np.float32)
    up = np.stack([np.array(Image.fromarray(c[..., i]).resize((w * k, h * k), Image.LANCZOS))
                   for i in range(c.shape[-1])], axis=-1)
    weight = np.ones(c.shape[-1])
    if c.shape[-1] == 4:
        weight[3] = 3.0
    rep = np.ones((k, k), np.int64)

    def dist(ix):
        d = np.atleast_3d(colour[ix]) - up
        return (d * d * weight).sum(-1)

    out = np.kron(idx, rep)
    best = dist(out)
    pad = np.pad(idx, 1, mode='edge')
    for dy in (-1, 0, 1):
        for dx in (-1, 0, 1):
            if dy or dx:
                cand = np.kron(pad[1 + dy:1 + dy + h, 1 + dx:1 + dx + w], rep)
                d = dist(cand)
                take = d < best
                out[take], best[take] = cand[take], d[take]
    return out


# ---- drawing glyphs ----------------------------------------------------------------


class Font:
    def __init__(self, path):
        self.f = ImageFont.truetype(path, 400)
        _, t, _, b = self.f.getbbox('H', anchor='ls')
        self.cap = b - t

    def ink(self, text):
        """`text`'s ink (0..255) cropped, and the baseline's row in it."""
        l, t, r, b = self.f.getbbox(text, anchor='ls')
        im = Image.new('L', (r - l + 8, b - t + 8), 0)
        ImageDraw.Draw(im).text((4 - l, 4 - t), text, font=self.f, fill=255, anchor='ls')
        a = np.array(im)
        ys, xs = np.nonzero(a >= 128)
        return a[ys.min():ys.max() + 1, xs.min():xs.max() + 1], (4 - t) - ys.min()


def icon(name, w, h, stroke):
    """The pad's buttons, outlined (a cross of two strokes)."""
    im = Image.new('L', (w, h), 0)
    dr = ImageDraw.Draw(im)
    s = max(1, int(round(stroke)))
    if name == 'circle':
        dr.ellipse([0, 0, w - 1, h - 1], outline=255, width=s)
    elif name == 'square':
        dr.rectangle([0, 0, w - 1, h - 1], outline=255, width=s)
    elif name == 'triangle':
        dr.polygon([(w / 2, 0), (w - 1, h - 1), (0, h - 1)], fill=255)
        t = s * 1.9
        ImageDraw.Draw(im).polygon([(w / 2, t * 1.1), (w - 1 - t, h - 1 - s), (t, h - 1 - s)], fill=0)
    elif name == 'cross':
        dr.line([(s / 2, s / 2), (w - s / 2, h - s / 2)], fill=255, width=s)
        dr.line([(w - s / 2, s / 2), (s / 2, h - s / 2)], fill=255, width=s)
    else:
        return None
    return np.array(im) >= 128


def boxed(font, text, w, h, stretch):
    """`text` fitted to w x h: its height, at most `stretch` times its own
    width, centred."""
    a, _ = font.ink(text)
    gw = min(w, max(1, int(round(a.shape[1] * h / a.shape[0] * stretch))))
    g = np.array(Image.fromarray(a).resize((gw, h), Image.LANCZOS)) >= 128
    full = np.zeros((h, w), bool)
    full[:, (w - gw) // 2:(w - gw) // 2 + gw] = g
    return full


def on_baseline(font, text, cap, base, h, fx0, fx1, s, descends):
    """`text` at the scale whose capitals' fill is `cap` texels, standing on
    cell row boundary `base`, centred between fx0 and fx1; squeezed about the
    baseline only as much as leaves a texel above and below for the ring, a
    mark too low for its cell lifted whole. None when it cannot fit."""
    a, b0 = font.ink(text)
    scale = cap * s / font.cap
    gw = min(max(1, int(round(a.shape[1] * scale))), (fx1 - fx0) * s)
    above, below = b0 * scale, (a.shape[0] - b0) * scale
    if below > s / 3 and not descends:
        above, below = above + below, 0.0
    base_at = base * s
    room_up, room_down = base_at - s, (h - 1) * s - base_at
    squeeze = min(1.0, room_up / above if above > 0 else 1.0,
                  room_down / below if below > s / 3 else 1.0)
    if squeeze < 0.75:
        squeeze = min(1.0, (h - 2) * s / (above + below))
    gh = max(1, int(round((above + below) * squeeze)))
    top = max(s, min(base_at - int(round(above * squeeze)), (h - 1) * s - gh))
    if top + gh > h * s:
        return None
    g = np.array(Image.fromarray(a).resize((gw, gh), Image.LANCZOS)) >= 128
    return top, fx0 * s + ((fx1 - fx0) * s - gw) // 2, g


def coverage(fill, w, h, k, weight):
    """Fill and ring (a page texel wide) coverage, k per texel, of a fill
    drawn SS times finer still; its strokes grown by `weight` texels."""
    s = k * SS
    if weight < 0:
        fill = ndimage.distance_transform_edt(fill) > -weight * s
    elif weight > 0:
        fill = ndimage.distance_transform_edt(~fill) <= weight * s
    ring = fill | (ndimage.distance_transform_edt(~fill) <= s)
    down = lambda m: m.reshape(h * k, SS, w * k, SS).mean(axis=(1, 3))
    return down(fill), down(ring)


def draw_cells(idx, glyphs, k, fonts, scheme, weight):
    """Each glyph of `glyphs` drawn from `fonts` (face -> (Font, stretch)) into
    a smooth enlargement of `idx`; `scheme` the page's indices."""
    out = smooth(idx, k, np.arange(256) * scheme['ramp'])
    s = k * SS
    fill_of = lambda cell: cell >= scheme['fill_from']
    # Each face's capitals' fill height, and the page's baseline (where a
    # glyph's record gives none).
    caps, bases = {}, []
    for g in glyphs:
        u, v, w, h = g['cell']
        if g['text'] in 'ABDEFHIKLMNPRTUVWXZ' and len(g['text']) == 1:
            rows = np.nonzero(fill_of(idx[v:v + h, u:u + w]).any(axis=1))[0]
            if len(rows):
                caps.setdefault(g.get('face'), []).append(rows.max() - rows.min() + 1)
                bases.append(rows.max() + 1)
    caps = {f: float(np.median(c)) for f, c in caps.items()}
    page_base = int(np.median(bases)) if bases else None
    # A texel around each cell first: the old ring may spill past it.
    for g in glyphs:
        u, v, w, h = g['cell']
        r = out[max(v - 1, 0) * k:(v + h + 1) * k, max(u - 1, 0) * k:(u + w + 1) * k]
        r[r != 0] = scheme['clear']
    drawn = 0
    for g in glyphs:
        text, face = g['text'], g.get('face')
        if not text or face not in fonts:
            continue
        u, v, w, h = g['cell']
        # A letter the sheet leaves blank (one PE2PC adds, `like` its model's
        # cell) goes where its model's ink is.
        lu, lv = g.get('like', g['cell'])[:2]
        cell = idx[lv:lv + h, lu:lu + w]
        ys, xs = np.nonzero(cell >= scheme['ink_from'])
        if len(ys) == 0:
            continue
        x0, y0, x1, y1 = xs.min(), ys.min(), xs.max() + 1, ys.max() + 1
        fx0, fy0, fx1, fy1 = x0 + 1, y0 + 1, x1 - 1, y1 - 1
        if fx1 <= fx0 or fy1 <= fy0:
            continue
        font, stretch = fonts[face]
        big = np.zeros((h * s, w * s), bool)
        placed = None
        if text.startswith('{'):
            shape = icon(text.strip('{}'), (fx1 - fx0) * s, (fy1 - fy0) * s, (2.0 if face == 'large' else 1.6) * s)
            if shape is None:
                continue
            big[fy0 * s:fy1 * s, fx0 * s:fx1 * s] = shape
        else:
            base = g.get('baseline', page_base)
            if face in caps and base is not None and stretch is None:
                rows = np.nonzero(fill_of(cell).any(axis=1))[0]
                descends = len(rows) and rows.max() + 1 > base
                placed = on_baseline(font, text, caps[face], base, h, fx0, fx1, s, descends)
            if placed:
                top, left, shape = placed
                big[top:top + shape.shape[0], left:left + shape.shape[1]] = shape
            else:
                big[fy0 * s:fy1 * s, fx0 * s:fx1 * s] = boxed(font, text, (fx1 - fx0) * s, (fy1 - fy0) * s,
                                                            stretch or 1.25)
        fc, oc = coverage(big, w, h, k, weight)
        keep0 = np.kron(cell == 0, np.ones((k, k), bool)) if scheme['keep_zero'] and 'like' not in g else False
        out[v * k:(v + h) * k, u * k:(u + w) * k] = np.where(keep0, 0, scheme['levels'](fc, oc))
        drawn += 1
    return out, drawn


# ---- the title's page --------------------------------------------------------------


def lzss(s, at):
    """One LZSS stream (dialogue.rs `lzss`)."""
    d, nxt, out = bytearray(256), 0, bytearray()
    state = [0, 0, at]  # byte, mask, position

    def bits(n):
        v = 0
        for _ in range(n):
            if state[1] == 0:
                if state[2] >= len(s):
                    return None
                state[0], state[1], state[2] = s[state[2]], 0x80, state[2] + 1
            v = (v << 1) | (1 if state[0] & state[1] else 0)
            state[1] >>= 1
        return v

    while (flag := bits(1)) is not None:
        if flag:
            b = bits(8)
            if b is None:
                break
            d[nxt] = b
            nxt = (nxt + 1) & 255
            out.append(b)
        else:
            off = bits(8)
            ln = bits(4) if off else None
            if ln is None:
                break
            fr = (off - 1) & 255
            for _ in range(ln + 2):
                b = d[fr]
                d[nxt] = b
                nxt = (nxt + 1) & 255
                out.append(b)
                fr = (fr + 1) & 255
    return bytes(out)


def palette(path):
    """A .pe2clut's first 256 colours as RGBA (colour 0 clear)."""
    c = open(path, 'rb').read()
    words = np.frombuffer(lzss(c, 16)[:512], dtype='<u2').astype(np.int64)
    rgba = np.zeros((256, 4), np.float64)
    for i, t in enumerate(words):
        rgba[i] = [(t & 31) << 3, (t >> 5 & 31) << 3, (t >> 10 & 31) << 3, 0 if t == 0 else 255]
    return rgba


def svg_shapes(path, max_x):
    """The SVG's outlines (paths: M, L, C, z, beziers in 16 steps; polygons)
    left of `max_x`: (black paths', black polygons', red ones), each a list
    of outlines of points. A path's outlines alternate (its holes); separate
    shapes add."""
    svg = open(path, encoding='utf-8').read()
    paths, polygons, red = [], [], []
    for m in re.finditer(r'<(path|polygon)\b([^>]*)>', svg):
        attrs = m.group(2)
        is_red = re.search(r'fill:\s*#(?:f00|ff0000)\b', attrs, re.I) is not None
        if m.group(1) == 'polygon':
            nums = [float(v) for v in re.findall(r'-?\d+(?:\.\d+)?', re.search(r'points="([^"]*)"', attrs).group(1))]
            outlines = [list(zip(nums[0::2], nums[1::2]))]
        else:
            outlines = []
            for part in re.split(r'(?=M)', re.search(r'\sd="([^"]*)"', attrs).group(1)):
                toks = re.findall(r'[MLCz]|-?\d+(?:\.\d+)?', part)
                pts, i, cmd = [], 0, None
                while i < len(toks):
                    if toks[i] in 'MLCz':
                        cmd, i = toks[i], i + 1
                    elif cmd in ('M', 'L'):
                        pts.append((float(toks[i]), float(toks[i + 1])))
                        i += 2
                    elif cmd == 'C':
                        p0, (c1, c2, p3) = pts[-1], [(float(toks[i + j]), float(toks[i + j + 1])) for j in (0, 2, 4)]
                        for n in range(1, 17):
                            t = n / 16
                            pts.append(tuple((1 - t) ** 3 * p0[a] + 3 * (1 - t) ** 2 * t * c1[a]
                                             + 3 * (1 - t) * t * t * c2[a] + t ** 3 * p3[a] for a in range(2)))
                        i += 6
                    else:
                        i += 1
                if len(pts) > 2:
                    outlines.append(pts)
        outlines = [o for o in outlines if min(x for x, _ in o) < max_x]
        (red if is_red else polygons if m.group(1) == 'polygon' else paths).extend(outlines)
    return paths, polygons, red


def border_only(band, c, clear):
    """Whether colour `c` of a band is only on its ink's outer rows and
    columns (a plate's frame)."""
    ys, xs = np.nonzero(band != clear)
    inner = band[ys.min() + 1:ys.max(), xs.min() + 1:xs.max()]
    return not (inner == c).any()


def title(args):
    m = find_map(args.dumps, 'title')
    idx = read_sheets(args.dumps, m)
    k = args.factor
    pal = palette(os.path.join(args.dumps, m['palette']))
    bright = pal[:, :3].mean(axis=1)
    known = pal[:, 3] > 0
    # Clear: the commonest colour of the rows of text.
    rows = np.concatenate([idx[g['cell'][1]:g['cell'][1] + g['cell'][3]].ravel() for g in m['glyph']])
    clear = int(np.bincount(rows).argmax())
    if args.over:
        # A mod's replacement of the page to start from: only the rows
        # given another text are redrawn.
        out = np.hstack([np.array(Image.open(os.path.join(args.over, f'{s}-{m["depth"]}bit.idx.png')))
                         for s in m['sheets']]).astype(np.int64)
        k = out.shape[0] // idx.shape[0]
    else:
        out = smooth(idx, k, pal)
    fonts = {'PRESS START BUTTON': args.bold, '© 1999 , 2000': args.serif}
    # --text "New Game=Novo Jogo": a row's text translated.
    translations = dict(t.split('=', 1) for t in args.text)
    for g in m['glyph']:
        u, v, w, h = g['cell']
        text = g['text']
        if text.startswith('{'):
            if args.logo:
                draw_logo(out, idx, (u, v, w, h), args.logo, pal, clear, k)
            continue
        font = fonts.get(text, args.regular)
        if font is None or (args.over and text not in translations):
            continue
        band = np.full((h, idx.shape[1]), clear)
        band[:, u:u + w] = idx[v:v + h, u:u + w]
        plate = None
        if text == 'PRESS START BUTTON':
            # Letters on a plate: the darkest of the colours where the
            # plate is (its border, too, is dark), then the brighter ones.
            on = band[band != clear]
            dark = [c for c in np.unique(on) if bright[c] < 40]
            plate = max(dark, key=lambda c: (on == c).sum() if not border_only(band, c, clear) else -1)
            mask = bright[band] > bright[plate] + 30
        else:
            mask = band != clear
        bg = clear if plate is None else plate
        ys, xs = np.nonzero(mask)
        y0, y1, x0, x1 = ys.min(), ys.max() + 1, xs.min(), xs.max() + 1
        letters = np.unique(band[mask])
        grey = lambda c: np.ptp(pal[c, :3]) <= 16
        shades = [c for c in letters if known[c] and grey(c)] + [bg]
        fill = max(letters, key=lambda c: bright[c])
        f = Font(font)
        cover, base = f.ink(text)
        # The font's scale that fits the original text to its ink.
        sx, sy = (x1 - x0) * k / cover.shape[1], (y1 - y0) * k / cover.shape[0]
        new = translations.get(text, text)
        if new != text:
            # Another text at the same scale, on the same baseline, centred
            # where the original was (within its cell).
            cover, new_base = f.ink(new)
            gw, gh = max(1, round(cover.shape[1] * sx)), max(1, round(cover.shape[0] * sy))
            top = round(y0 * k + base * sy - new_base * sy)
            left = min(max(round((x0 + x1) * k / 2 - gw / 2), u * k), (u + w) * k - gw)
        else:
            gw, gh, top, left = (x1 - x0) * k, (y1 - y0) * k, y0 * k, x0 * k
        cover = np.array(Image.fromarray(cover).resize((gw, gh), Image.LANCZOS)) / 255.0
        target = bright[bg] + cover * (bright[fill] - bright[bg])
        vals = np.array([bright[c] for c in shades])
        pick = np.array(shades)[np.abs(target[..., None] - vals).argmin(-1)]
        # The old ink, and a texel around it, cleared; then the new.
        near = ndimage.binary_dilation(mask, np.ones((3, 3), bool))
        near[:, :u] = near[:, u + w:] = False
        region = out[v * k:(v + h) * k]
        region[np.kron(near, np.ones((k, k), bool))] = bg
        # What of the new ink falls inside the row.
        cy0, cx0 = max(0, -top), max(0, -left)
        cy1, cx1 = min(gh, region.shape[0] - top), min(gw, region.shape[1] - left)
        if plate is not None and new != text:
            # Other words need their own plate: the old one cleared, and a
            # box behind each new word (a texel around its ink).
            whole = np.zeros_like(band, bool)
            whole[:, u:u + w] = band[:, u:u + w] != clear
            region[np.kron(whole, np.ones((k, k), bool))] = clear
            cols = (cover[cy0:cy1, cx0:cx1] > 0.02).any(axis=0)
            gap = round(0.4 * gh)
            xs = np.nonzero(cols)[0]
            words, start = [], xs[0]
            for a, b in zip(xs, xs[1:]):
                if b - a > gap:
                    words.append((start, a + 1))
                    start = b
            words.append((start, xs[-1] + 1))
            for a, b in words:
                y_a, y_b = max(0, top + cy0 - k), min(region.shape[0], top + cy1 + k)
                x_a, x_b = max(0, left + cx0 + a - k), min(region.shape[1], left + cx0 + b + k)
                region[y_a:y_b, x_a:x_b] = plate
        part = region[top + cy0:top + cy1, left + cx0:left + cx1]
        ink = cover[cy0:cy1, cx0:cx1] > 0.02
        part[ink] = pick[cy0:cy1, cx0:cx1][ink]
        print(f'{new!r}: {gw / k:.0f}x{gh / k:.0f} texels at ({left / k:.0f}, {v + top / k:.0f})')
    write_sheets(args.out, m, out, k, 'title text redrawn from vector fonts')


def draw_logo(out, idx, cell, svg, pal, clear, k):
    """The logo from `svg` (white, its red parts red) in its cell's ink box,
    keeping its proportions, each finer texel the nearest of the colours
    the old logo uses (drawn added to the screen: black is nothing)."""
    u, v, w, h = cell
    paths, polygons, red = svg_shapes(svg, 320)
    pts = [p for o in paths + polygons for p in o]
    sx0, sx1 = min(x for x, _ in pts), max(x for x, _ in pts)
    sy0, sy1 = min(y for _, y in pts), max(y for _, y in pts)
    area = idx[v:v + h, u:u + w] != clear
    ys, xs = np.nonzero(area)
    x0, x1, y0, y1 = u + xs.min(), u + xs.max() + 1, v + ys.min(), v + ys.max() + 1
    bw, bh = (x1 - x0) * k, (y1 - y0) * k
    scale = min(bw / (sx1 - sx0), bh / (sy1 - sy0))
    ox, oy = (bw - (sx1 - sx0) * scale) / 2, (bh - (sy1 - sy0) * scale) / 2

    def cover(outlines, even_odd):
        acc = np.zeros((bh * SS, bw * SS), bool)
        for o in outlines:
            im = Image.new('1', (bw * SS, bh * SS), 0)
            ImageDraw.Draw(im).polygon([(((x - sx0) * scale + ox) * SS, ((y - sy0) * scale + oy) * SS) for x, y in o],
                                       fill=1)
            acc = acc ^ np.array(im) if even_odd else acc | np.array(im)
        return acc.reshape(bh, SS, bw, SS).mean(axis=(1, 3))

    white = np.maximum(cover(paths, True), cover(polygons, False))
    redm = cover(red, False)
    rgb = white[..., None] * np.array([248.0, 248, 248])
    rgb = rgb * (1 - redm[..., None]) + redm[..., None] * np.array([248.0, 32, 48])
    used = np.array([c for c in np.unique(idx[y0:y1, x0:x1]) if c == clear or pal[c, 3] > 0])
    colours = np.where(used[:, None] == clear, 0, pal[used, :3])
    out[y0 * k:y1 * k, x0 * k:x1 * k] = used[((rgb[..., None, :] - colours) ** 2).sum(-1).argmin(-1)]
    print(f'logo: {x1 - x0}x{y1 - y0} texels at ({x0}, {y0})')


# ---- the commands ------------------------------------------------------------------


def ui(args):
    m = find_map(args.dumps, 'ui-font')
    idx = read_sheets(args.dumps, m)
    font = Font(args.font)
    # The small face is wide capitals: stretched into its cells.
    fonts = {'medium': (font, None), 'large': (font, None), 'small': (font, 3.0)}
    scheme = {'ramp': 17, 'clear': 5, 'fill_from': 13, 'ink_from': 8, 'keep_zero': True,
              'levels': lambda f, o: np.rint(5 + 5 * o + 5 * f).astype(np.int64).clip(5, 15)}
    out, drawn = draw_cells(idx, m['glyph'], args.factor, fonts, scheme, args.weight)
    print(f'{drawn} glyphs drawn')
    write_sheets(args.out, m, out, args.factor, 'UI font redrawn from a vector font')


def loading(args):
    fonts = {None: (Font(args.font), None)}

    def levels(f, o):
        return np.where(o >= 0.5, np.rint(1 + 14 * f).astype(np.int64).clip(1, 15), 0)

    scheme = {'ramp': 17, 'clear': 0, 'fill_from': 10, 'ink_from': 1, 'keep_zero': False, 'levels': levels}
    # The loading screen's sheet, and the M.I.S.T. briefing's copy of its font.
    for m in find_maps(args.dumps, 'loading'):
        idx = read_sheets(args.dumps, m)
        out, drawn = draw_cells(idx, m['glyph'], args.factor, fonts, scheme, args.weight)
        print(f'{drawn} glyphs drawn')
        write_sheets(args.out, m, out, args.factor, 'loading captions redrawn from a vector font')


def main():
    p = argparse.ArgumentParser(description=__doc__.split('\n\n')[0])
    sub = p.add_subparsers(dest='command', required=True)
    for name, weight in (('ui', -0.25), ('loading', 0.15)):
        s = sub.add_parser(name)
        s.add_argument('dumps')
        s.add_argument('font')
        s.add_argument('out')
        s.add_argument('--factor', type=int, default=8)
        s.add_argument('--weight', type=float, default=weight, help='texels the strokes grow (negative: thinner)')
    s = sub.add_parser('title')
    s.add_argument('dumps')
    s.add_argument('regular')
    s.add_argument('bold')
    s.add_argument('out')
    s.add_argument('--serif', help='font for the copyright line (else it is kept)')
    s.add_argument('--logo', help='SVG of the logo (else it is kept)')
    s.add_argument('--over', metavar='DIR', help="a mod's textures to start from (its title page's replacements)")
    s.add_argument('--text', action='append', default=[], metavar='ORIGINAL=NEW',
                   help='draw a row in another text, e.g. "New Game=Novo Jogo" (repeatable)')
    s.add_argument('--factor', type=int, default=8)
    args = p.parse_args()
    {'ui': ui, 'loading': loading, 'title': title}[args.command](args)


if __name__ == '__main__':
    main()
