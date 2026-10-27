# Gba-game

This is a game I'm making for terra using AGB and rust.

## Controls
Using the d-pad left and right buttons will make you move left and right.
Pressing the A button while you're on the ground will make you jump.
Pressing the A button in the water will make you swim.
Touching a wall while in the air, with the A button released and holding the direction of the wall on the d-pad will make you slide.
While sliding, you can press the opposite direction on the dpad to stop sliding or press the A button to perform a wall jump.

## Building
The requirements are:
- Cargo
- arm-none-eabi gnu toolchain

Just run:

```shell
cargo build
```

or

```shell
cargo build --release
```

for building with release optimizations.

## Running

Run:

```shell
cargo run
```

in order to run it in development. (Add --release for the release version)

A GBA emulator is required to run on pc. I use mGBA, as it is the recommended one by agb and it's pretty accurate.

If you need to run it on actual hardware you must first install agb-gbafix:

```shell
cargo install agb-gbafix
```

And then run (after building for release):

```shell
agb-gbafix target/thumbv4t-none-eabi/release/gba-game -o gba-game.gba
```

the output (gba-game.gba) will pass all validations and run on real hardware.

I personally tested this on my 2 gbas using a flashcard running superfw and it works flawlessly.
