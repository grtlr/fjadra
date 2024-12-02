# Fjädra 🪽

A library for simulating physical forces on particles, which was heavily inspired by [`d3-force`](https://d3js.org/d3-force).
Its main use case is to layout graphs (i.e. node-link diagrams)—if you are looking for a pure physics engine, you might want to check out the excellent [Rapier](https://rapier.rs/) and [Parry](https://parry.rs/) from [Dimforge](`www.dimforge.com`).

> [!WARNING]
> This library is currently under development so the API is still likely to change.

---

<div align="center">
<a href="https://www.rerun.io/"><img src="media/rerun_io_logo.png" width="250"></a>

Development is sponsored by [Rerun](https://www.rerun.io/), a startup building<br>
an SDK for visualizing streams of multimodal data.
</div>

---

## Design Goals

- Produce outputs that are comparable to `d3-force`.
- Lightweight with only minimal dependencies to allow `wasm-bindgen` and `no_std` support.
- Idiomatic Rust API that still follows the `d3` conventions where possible.
- Performance that allows user interactions.

## Forces

We currently support the following forces:

- Collision on circles (`Collide`)
- Centering (`Center`)
- Springs (`Link`)
- Charge and repulsion (`ManyBody`)
- Gravity (`PositionX` and `PositionY`)
