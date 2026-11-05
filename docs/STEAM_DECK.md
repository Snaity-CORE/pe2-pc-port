# Steam Deck and Linux

PE2PC is a Windows program, and it runs on the Steam Deck and other
Linux machines through Proton, the same way Steam runs Windows games.
No separate Linux build is needed: use the normal Windows release.

It was tested with Proton 10.0 on a Bazzite PC (Steam's Gaming Mode and
the KDE desktop). The steps are the same on SteamOS.

> **Not yet tested on a real Steam Deck.** On the test PC, a low-power
> one (a 6 W, two-core AMD Athlon Silver 3050e), the 3D scenes ran slower
> than normal speed; the Deck's processor is much stronger, but how it
> runs there is not known yet. If you play it on a Deck, or hit any
> problem, please
> [open an issue](https://github.com/Snaity-CORE/pe2-pc-port/issues)
> with your device, your Proton version and the game's `pe2.log`.

## What you need

- The latest release zip,
  `pe2-pc-port-vX.Y.Z-windows-x86.zip`, from the
  [releases page](https://github.com/Snaity-CORE/pe2-pc-port/releases/latest).
- Your own dumps of Parasite Eve II (USA), discs 1 and 2, as `.chd` or
  `.cue`/`.bin` images, as on Windows.

## 1. Put the files in place (Desktop Mode)

Switch to Desktop Mode (Steam button > Power > Switch to Desktop).

1. Make a folder for the game, for example `/home/deck/Games/PE2PC`.
2. Extract the zip there.
3. Create an empty file named `portable.txt` in that folder (in Dolphin,
   right-click > Create New > Text File). With it, your settings,
   memory cards and screenshots stay in this folder. Without it, they
   go into Proton's own folder for the game, deep inside
   `~/.local/share/Steam/steamapps/compatdata/`, which is hard to find.
4. Copy your disc images next to it, for example into
   `/home/deck/Games/PE2PC/discs/`. A microSD card works too.

## 2. Add it to Steam

1. In Desktop Mode, open Steam and choose **Games > Add a Non-Steam
   Game to My Library**.
2. Click **Browse**, set the file type to **All Files**, and pick
   `pe2.exe` in the game's folder.
3. In your library, open the new entry's **Properties**:
   - Name it as you like (for example *Parasite Eve II*).
   - Under **Compatibility**, tick **Force the use of a specific Steam
     Play compatibility tool** and pick **Proton 10.0** (Proton
     Experimental should also work).

## 3. First start: Setup

Start the game from Steam. The launcher opens with its Setup screens:
the language, the disc images, the memory cards and a quality level.

The disc picker is a Windows file dialog. Your Linux files are on drive
**Z:**, so your home folder is `Z:\home\deck`. A microSD card is under
`Z:\run\media\deck` (or `Z:\run\media\mmcblk0p1` on older SteamOS).

The file dialog is easiest with a mouse and keyboard, so do Setup once
in Desktop Mode. You can also skip the dialog: copy `pe2.example.toml`
to `pe2.toml` in the game's folder and set the discs in it (use your own
file names):

```toml
[discs]
disc1 = 'Z:\home\deck\Games\PE2PC\discs\Parasite Eve II (USA) (Disc 1).chd'
disc2 = 'Z:\home\deck\Games\PE2PC\discs\Parasite Eve II (USA) (Disc 2).chd'
```

## 4. Play (Gaming Mode)

Back in Gaming Mode, start the game from your library.

- **Deck mode.** In Gaming Mode the launcher opens in full screen, sized
  to the screen, and the controls work it: d-pad or left stick to move,
  A to choose, B to go back. You can turn it always on or off in
  **Settings > General > Deck mode**.
- **Controls.** The Deck's buttons work as an Xbox controller: A = Cross,
  B = Circle, X = Square, Y = Triangle, L1/R1/L2/R2 on the bumpers and
  triggers, and View/Menu as Select/Start. Keep Steam Input on its
  default *Gamepad* layout. **Controls** in the launcher remaps the
  buttons.
- **Pause menu.** Hold **Start + Select** together (the Steam button
  belongs to Steam). **Controls > Controller > Pause menu** can change it
  to L3 + R3.
- **Quality.** The Deck's screen is 1280×800. An internal resolution of
  3× (960×720) or 4× is sharp there; lower it in **Settings > Graphics**
  if the game does not keep a steady speed.

## Without Steam

On a Linux desktop you can also run it with `umu-run` (Bazzite has it;
Lutris and Heroic use it too), from the game's folder:

```sh
WINEPREFIX="$PWD/prefix" PROTONPATH="$HOME/.local/share/Steam/steamapps/common/Proton 10.0" GAMEID=umu-default umu-run ./pe2.exe
```

## If something goes wrong

- **No controller in the game.** Check that Steam Input is on for the
  game. The launcher's **Controls** screen lists the controllers it
  sees, as "XInput controller 1" and so on.
- **A black screen or a crash at start.** In **Settings > Graphics**, set
  the graphics API to OpenGL instead of Automatic.
- **Anything else.** The game writes `pe2.log` in its data folder (the
  game's folder, with `portable.txt`). Attach it to an
  [issue](https://github.com/Snaity-CORE/pe2-pc-port/issues).
