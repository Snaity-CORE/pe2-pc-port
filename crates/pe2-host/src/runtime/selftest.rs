//! `PE2_SELFTEST=1`: checks the interpreter and the software GTE by running
//! the game's own math library routines (from the main executable in PSX
//! RAM) on known inputs and comparing with independently computed results.
//! Runs after the executable is loaded, before the game starts, then exits.

use super::mips::pc_mips_call;

/// Scratch area for arguments, above the main executable's BSS.
const BASE: u32 = 0x8010_0000;

struct Lib {
    rsin: u32,
    rcos: u32,
    rot_matrix: u32,
    mul_matrix0: u32,
    apply_matrix_lv: u32,
    scale_matrix: u32,
    square_root0: u32,
    ratan2: u32,
    transpose: u32,
}

fn call(addr: u32, args: &[u32]) -> u32 {
    let mut a = [0u32; 8];
    a[..args.len()].copy_from_slice(args);
    // SAFETY: `a` holds eight words; `addr` is library MIPS code in RAM.
    unsafe { pc_mips_call(addr, a.as_ptr()) }
}

fn w16(addr: u32, v: i16) {
    // SAFETY: BASE.. is mapped PSX RAM.
    unsafe { (addr as *mut i16).write_unaligned(v) }
}
fn w32(addr: u32, v: i32) {
    // SAFETY: as above.
    unsafe { (addr as *mut i32).write_unaligned(v) }
}
fn r16(addr: u32) -> i16 {
    // SAFETY: as above.
    unsafe { (addr as *const i16).read_unaligned() }
}
fn r32(addr: u32) -> i32 {
    // SAFETY: as above.
    unsafe { (addr as *const i32).read_unaligned() }
}

/// MATRIX { short m[3][3]; long t[3]; } (m at 0, t at 20).
fn write_matrix(addr: u32, m: [[i16; 3]; 3], t: [i32; 3]) {
    for (r, row) in m.iter().enumerate() {
        for (c, v) in row.iter().enumerate() {
            w16(addr + ((r * 3 + c) * 2) as u32, *v);
        }
    }
    w16(addr + 18, 0);
    for (i, v) in t.iter().enumerate() {
        w32(addr + 20 + 4 * i as u32, *v);
    }
}

fn read_matrix(addr: u32) -> [[i16; 3]; 3] {
    let mut m = [[0i16; 3]; 3];
    for (r, row) in m.iter_mut().enumerate() {
        for (c, v) in row.iter_mut().enumerate() {
            *v = r16(addr + ((r * 3 + c) * 2) as u32);
        }
    }
    m
}

struct Report {
    failures: u32,
}

impl Report {
    fn check(&mut self, what: &str, got: i64, want: i64, tolerance: i64) {
        if (got - want).abs() > tolerance {
            self.failures += 1;
            eprintln!("[selftest] FAIL {what}: got {got}, want {want}");
        }
    }
}

/// Deterministic pseudo-random numbers for test inputs.
struct Rng(u32);
impl Rng {
    fn next(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(1_103_515_245).wrapping_add(12345);
        self.0 >> 8
    }
    fn range(&mut self, lo: i32, hi: i32) -> i32 {
        lo + (self.next() % (hi - lo + 1) as u32) as i32
    }
}

fn mul_ref(a: [[i16; 3]; 3], b: [[i16; 3]; 3]) -> [[i64; 3]; 3] {
    let mut out = [[0i64; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            let sum: i64 = (0..3).map(|k| i64::from(a[i][k]) * i64::from(b[k][j])).sum();
            out[i][j] = (sum >> 12).clamp(-0x8000, 0x7FFF);
        }
    }
    out
}

pub fn run(lib_addr: impl Fn(&str) -> Option<u32>) -> u32 {
    let get = |n: &str| lib_addr(n).unwrap_or_else(|| panic!("selftest: no address for {n}"));
    let lib = Lib {
        rsin: get("rsin"),
        rcos: get("rcos"),
        rot_matrix: get("RotMatrix"),
        mul_matrix0: get("MulMatrix0"),
        apply_matrix_lv: get("ApplyMatrixLV"),
        scale_matrix: get("ScaleMatrix"),
        square_root0: get("SquareRoot0"),
        ratan2: get("ratan2"),
        transpose: get("TransposeMatrix"),
    };
    let mut rep = Report { failures: 0 };
    let mut rng = Rng(0x1234_5678);

    // rsin / rcos: 4096 = 1.0, 4096 units per turn.
    for a in [0, 256, 512, 1024, 1536, 2048, 3000, 4095, -700, 9000] {
        let s = call(lib.rsin, &[a as u32]) as i32;
        let c = call(lib.rcos, &[a as u32]) as i32;
        let rad = f64::from(a) * std::f64::consts::TAU / 4096.0;
        rep.check(&format!("rsin({a})"), s.into(), (rad.sin() * 4096.0).round() as i64, 2);
        rep.check(&format!("rcos({a})"), c.into(), (rad.cos() * 4096.0).round() as i64, 2);
    }

    // ratan2(y, x) in 4096 units per turn.
    for (y, x) in [(0, 100), (100, 0), (100, 100), (-50, 200), (300, -400), (-7, -7)] {
        let got = call(lib.ratan2, &[y as u32, x as u32]) as i32;
        let want = (f64::from(y).atan2(f64::from(x)) * 4096.0 / std::f64::consts::TAU).round() as i64;
        rep.check(&format!("ratan2({y},{x})"), got.into(), want, 3);
    }

    // SquareRoot0(x) = floor(sqrt(x)).
    for x in [0u32, 1, 2, 15, 16, 1000, 65535, 1_000_000, 0x7FFF_FFFF] {
        let got = call(lib.square_root0, &[x]) as i32;
        rep.check(&format!("SquareRoot0({x})"), got.into(), f64::from(x).sqrt().floor() as i64, 1);
    }

    // RotMatrix of zero angles is the identity; any rotation matrix has
    // orthonormal rows (|row| = 4096, rows orthogonal).
    let (sv, m0, m1, m2, v0, v1) = (BASE, BASE + 0x40, BASE + 0x80, BASE + 0xC0, BASE + 0x100, BASE + 0x140);
    for case in 0..12 {
        let ang = if case == 0 { [0, 0, 0] } else { [rng.range(-4096, 4096), rng.range(-4096, 4096), rng.range(-4096, 4096)] };
        for (i, a) in ang.iter().enumerate() {
            w16(sv + 2 * i as u32, *a as i16);
        }
        write_matrix(m0, [[0; 3]; 3], [0; 3]);
        call(lib.rot_matrix, &[sv, m0]);
        let m = read_matrix(m0);
        if case == 0 {
            for (i, row) in m.iter().enumerate() {
                for (j, v) in row.iter().enumerate() {
                    rep.check(&format!("RotMatrix(0) [{i}][{j}]"), (*v).into(), if i == j { 4096 } else { 0 }, 0);
                }
            }
        }
        for i in 0..3 {
            let len: f64 = m[i].iter().map(|v| f64::from(*v).powi(2)).sum::<f64>().sqrt();
            rep.check(&format!("RotMatrix{ang:?} |row {i}|"), len.round() as i64, 4096, 8);
            for j in i + 1..3 {
                let dot: i64 = (0..3).map(|k| i64::from(m[i][k]) * i64::from(m[j][k])).sum::<i64>() >> 12;
                rep.check(&format!("RotMatrix{ang:?} row{i}.row{j}"), dot, 0, 8);
            }
        }
    }

    // MulMatrix0(a, b, out): out = a * b >> 12, saturated to 16 bits.
    for case in 0..20 {
        let lim = if case < 10 { 4096 } else { 0x7FFF };
        let mut a = [[0i16; 3]; 3];
        let mut b = [[0i16; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                a[i][j] = rng.range(-lim, lim) as i16;
                b[i][j] = rng.range(-lim, lim) as i16;
            }
        }
        write_matrix(m0, a, [0; 3]);
        write_matrix(m1, b, [0; 3]);
        call(lib.mul_matrix0, &[m0, m1, m2]);
        let got = read_matrix(m2);
        let want = mul_ref(a, b);
        for i in 0..3 {
            for j in 0..3 {
                rep.check(&format!("MulMatrix0 #{case} [{i}][{j}]"), got[i][j].into(), want[i][j], 0);
            }
        }
    }

    // TransposeMatrix(m, out).
    let mut a = [[0i16; 3]; 3];
    for row in a.iter_mut() {
        for v in row.iter_mut() {
            *v = rng.range(-4096, 4096) as i16;
        }
    }
    write_matrix(m0, a, [0; 3]);
    call(lib.transpose, &[m0, m1]);
    let t = read_matrix(m1);
    for i in 0..3 {
        for j in 0..3 {
            rep.check(&format!("TransposeMatrix [{i}][{j}]"), t[i][j].into(), a[j][i].into(), 0);
        }
    }

    // ApplyMatrixLV(m, v0, v1): v1 = m * v0 >> 12 on 32-bit vectors.
    for case in 0..20 {
        let mut a = [[0i16; 3]; 3];
        for row in a.iter_mut() {
            for v in row.iter_mut() {
                *v = rng.range(-4096, 4096) as i16;
            }
        }
        let lim = if case < 10 { 0x7FFF } else { 0x7F_FFFF };
        let v = [rng.range(-lim, lim), rng.range(-lim, lim), rng.range(-lim, lim)];
        write_matrix(m0, a, [0; 3]);
        for (i, x) in v.iter().enumerate() {
            w32(v0 + 4 * i as u32, *x);
        }
        w32(v0 + 12, 0);
        call(lib.apply_matrix_lv, &[m0, v0, v1]);
        for (i, row) in a.iter().enumerate() {
            let want: i64 = (0..3).map(|k| i64::from(row[k]) * i64::from(v[k])).sum::<i64>() >> 12;
            rep.check(&format!("ApplyMatrixLV #{case} [{i}] v={v:?}"), r32(v1 + 4 * i as u32).into(), want, 2);
        }
    }

    // ScaleMatrix(m, VECTOR s): column j multiplied by s[j] >> 12.
    for case in 0..10 {
        let mut a = [[0i16; 3]; 3];
        for row in a.iter_mut() {
            for v in row.iter_mut() {
                *v = rng.range(-4096, 4096) as i16;
            }
        }
        let s = [rng.range(0, 8192), rng.range(0, 8192), rng.range(0, 8192)];
        write_matrix(m0, a, [0; 3]);
        for (i, x) in s.iter().enumerate() {
            w32(v0 + 4 * i as u32, *x);
        }
        call(lib.scale_matrix, &[m0, v0]);
        let got = read_matrix(m0);
        for i in 0..3 {
            for j in 0..3 {
                let want = (i64::from(a[i][j]) * i64::from(s[j])) >> 12;
                rep.check(&format!("ScaleMatrix #{case} [{i}][{j}]"), got[i][j].into(), want, 1);
            }
        }
    }

    eprintln!("[selftest] {} failure(s)", rep.failures);
    rep.failures
}
