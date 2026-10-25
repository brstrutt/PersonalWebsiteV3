---
title: "Roll of the die"
date: 2022-07-17
lastmod: 2023-04-08
tags:
- Rust
- WASM
- Bevy
- Game Jam
- Game
- Video Games
---

## Roll the Die

This is a game produced during GMTK Game Jam 2022 as an excuse to mess around with building web games using Rust.

The game can be played bey going to [https://brstrutt.github.io/Roll-of-the-Die/](https://brstrutt.github.io/Roll-of-the-Die/).

## How was this built?

After stumbling around in the dark for a few hours I found out about a small game engine for Rust called [Bevy](https://bevyengine.org). The main selling point for me was Bevy's ability to build for a WASM target natively. This made life MUCH easier than I expected.

The only other tool I really needed (other than the standard Rust toolchain) was Trunk for handling building and running a small web server for testing.

I have to stop writing this now but if you hit up [The Repo](https://github.com/brstrutt/Roll-of-the-Die) you can see the source and there should be some basic build/run instructions if you want them.
