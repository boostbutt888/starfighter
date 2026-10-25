# Starfighter · Rebel Command

A Star Wars themed vertical shooter written in **Rust**. The Windows app and phone-browser build share the same simulation, rendering and synthesized audio through Macroquad and WebAssembly.

**[Play Starfighter](https://boostbutt888.github.io/starfighter/)**

## Play

- Windows: double-click `Play.cmd`; it selects the newest `Starfighter-Latest.exe`.
- Phone: use the online link in portrait orientation. The field extends to fit tall screens and respects safe areas.
- Local Wi-Fi: run `Play-on-Phone.cmd` on the computer and open the printed phone address on the same network. Keep the server running.

## Controls

The arrangement follows 194X: **Pause / Play**, **Swap** and **Mute / Sound** at top right, a floating **BOMB** button at bottom right, and the HUD at top left. Drag to steer with automatic fire. Movement is relative, so landing a finger never teleports the fighter. A second finger can launch a bomb without disturbing steering. Backgrounding the browser pauses play.

| Keyboard | Action |
| --- | --- |
| WASD / arrows | Move |
| Space / Z | Fire |
| C / Tab | Swap Spread / Cannon |
| B / X | Bomb |
| Left Shift | Precision movement |
| P / Escape | Pause / resume |
| M | Toggle audio |
| F11 | Fullscreen |
| Left / right at menu | Select craft |
| 1 / 2 / 3 | Rookie / Ace / Legend |
| Q / E or Page Up / Down | Select deployment |
| Enter / Space | Launch / return after debrief |

## Rebel craft

All three are selectable immediately and rendered from overhead at reduced fighter scale.

| Craft | Handling and lasers | Starting bombs |
| --- | --- | --- |
| X-wing | Balanced; four wing cannons; red | 3 |
| Y-wing | Slower, stronger Force capacity; nose cannons; blue | 5 |
| A-wing | Fastest and most agile; twin edge cannons; green | 2 |

Glowing blue **THE FORCE** pickups restore 35 Force and add a bomb. Spread and Cannon each have six independent upgrade levels. Spread widens into a fan; Cannon gains width, damage and piercing. A lost craft is replaced by a reserve wingman with restored Force, brief protection, at least two bombs and one weapon level lost.

## Five missions

| Mission | Boss | Turrets |
| --- | --- | --- |
| Encounter at Tatooine | Jabba's sail barge; Jabba is the protected command target | 10 |
| Siege of Hoth | Executor | 16 |
| Shadows over Dagobah | Two Star Destroyers | 24 |
| Assault on Coruscant | Death Star I and two Star Destroyers | 40 |
| Battle above Endor | Death Star II, then a surprise fleet of two Star Destroyers and Executor | 24, then 40 |

Endor approaches during the scrolling final mission; Death Star II arrives when the approach completes. Scrolling continues through both boss phases. This is an arcade fantasy campaign rather than a canonical chronology.

Bosses face down toward the player. Every turret rotates, tracks, fires, recoils, takes damage and leaves a silent wreck independently. **Destroy every gun in the fleet before attacking the command targets.** A bomb that removes the last gun cannot damage a bridge in the same blast. Total boss armor increases each mission, and the final surprise fleet exceeds Death Star II's armor. Warnings appear briefly at center without announcement boxes.

Dive, side-sweep and turning-hook flight paths, tangent rotation, wave cadence, six-level weapons, the 0.65-second bomb run, difficulty scaling and touch placement closely follow the earlier 194X game. Tough TIE bombers add radial starbursts. Lasers aim across the field in any direction.

## Art and music

Realistic rendered terrain, transparent overhead fighters, capital ships, Death Stars, Jabba's barge and turret sprites were created with built-in imagegen. Planetary textures are 1024 × 1536. See [the final art prompts](assets/GENERATION.md).

**Dawn of the Rebellion** is an original 16-bar orchestral-style space-adventure march, synthesized in Rust, with a faster boss arrangement. Brass-like lead, layered strings, bass, snare and timpani-style percussion accompany pitch-sweeping lasers and explosions with descending sub-bass. No film recordings or existing Star Wars music are used. Menu music and effects toggles are independent.

## Build

Install Rust, the `wasm32-unknown-unknown` target and a platform linker. `Build.ps1` builds the native game, Rust local server and browser package, and recognizes the sibling 194x project's MinGW toolchain.

```text
cargo test
cargo build --release --bin starfighter
cargo build --release --target wasm32-unknown-unknown --bin starfighter
```

Serve `web/` with a static host or the bundled Rust server; direct `file://` WebAssembly loading is unsupported. The browser's JavaScript is a small WebAssembly host, not a separate gameplay implementation.

Tests cover craft loadouts, weapons, Force, flight paths, bomber starbursts, boss armor, turret tracking and destruction, protected bridges, bomb ordering, projectile-batch safety, the final ambush, Endor arrival, phone height, rapid taps and two-finger controls. Chromium tests also exercise the compiled Rust browser game on desktop and at phone pixel ratio 3, including audio playback.

## Credits

Research: [Galactic Empire Databank](https://www.starwars.com/databank/organization/theempire/), [Executor](https://www.starwars.com/databank/executor/), [A-wing](https://www.starwars.com/databank/a-wing-fighter), [Death Star](https://www.starwars.com/databank/Death-Star). Barlow fonts include their [SIL Open Font License](assets/fonts/OFL.txt).

Unofficial fan game. Star Wars names, characters and vehicles belong to their respective owners.
