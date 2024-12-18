<h1>
  Fjädra 🪽
  <a href="https://crates.io/crates/fjadra">                            <img alt="crates.io"      src="https://img.shields.io/crates/v/fjadra.svg">                               </a>
  <a href="https://github.com/rerun-io/rerun/blob/main/LICENSE-MIT">    <img alt="MIT"            src="https://img.shields.io/badge/license-MIT-blue.svg">                        </a>
  <a href="https://github.com/rerun-io/rerun/blob/main/LICENSE-APACHE"> <img alt="Apache"         src="https://img.shields.io/badge/license-Apache-blue.svg">                     </a>
</h1>

[**Documentation**](https://docs.rs/fjadra/latest/fjadra/)

`fjadra` is a library for simulating physical forces on particles, which was heavily inspired by [`d3-force`](https://d3js.org/d3-force).
Its main use case is to layout graphs (i.e. node-link diagrams)—if you are looking for a pure physics engine, you might want to check out the excellent [Rapier](https://rapier.rs/) and [Parry](https://parry.rs/) from [Dimforge](`www.dimforge.com`).

---

<div align="center">
<a href="https://www.rerun.io/"><img src="media/rerun_io_logo.png" width="250"></a>

Development is sponsored by [Rerun](https://www.rerun.io/), a startup building<br>
an SDK for visualizing streams of multimodal data.
</div>

---

## Design Goals

- Produce outputs that are comparable to `d3-force`.
- Lightweight with only minimal dependencies to allow `wasm-bindgen` and future `no_std` support.
- Idiomatic Rust API that still follows the `d3` conventions where possible.
- Performance that allows user interactions.

## Forces

We currently support the following forces:

- Collision on circles (`Collide`)
- Centering (`Center`)
- Springs (`Link`)
- Charge and repulsion (`ManyBody`)
- Gravity (`PositionX` and `PositionY`)
