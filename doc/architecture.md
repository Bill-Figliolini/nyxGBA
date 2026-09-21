# Architecture

## CPU

The GBA runs off of a single core 32bit CPU, running 16 registers and two instruction sets: ARM (32 bit) and THUMB (16 bit).

## Memory

2 primary regions: Internal, and External.
External is the ROM loaded by the user.
Internal is broken up into:
General
BIOS - 16K
Work RAM - 32K Fast, 256K Slow
Display
VRAM - 96K
OAM - 1K
Pallette RAM - 1K
Exclusively little endian

## IO

TODO

## Display

TODO

## Sound

TODO
