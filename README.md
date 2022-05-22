# ripp8
CHIP8 interpreter created in Rust to get a grasp on the language, learn extra crates and libraries, and as an introduction to the field of emulation.

How to run: clone this directory, run "cargo build --release" if you have Rust installed, and run "./target/release/ripp8"
Currently is hard coded to run PONG2 and mimics the exact keys the CHIP8 keyboard used.
You control the left player with 1 and 4, the right player with D and C. 
Currently only runs with the hardcoded path on my system. This will be changed soon.

TODOS:
Add command line arguments for window scale, path to rom, and game speed.
Test and fix compatibility issues with some CHIP8 roms.
Create statically compiled executables for at least Linux, other platforms later to test cross-platform compatibility.
Change control scheme to better match the layout of the original CHIP8 keyboards.

Documentation used: Cowgod's technical reference for CHIP-8: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
