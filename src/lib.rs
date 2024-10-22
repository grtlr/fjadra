//! # Fjädra
//!
//! Library for simulating physical forces on particles.
//! Heavily inspired by [`d3-force`](https://d3js.org/d3-force).
//!
//! We currently support the following forces:
//!
//! * [`Collide`]
//! * [`Link`]
//! * [`ManyBody`]
//! * [`PositionX`] and [`PositionY`]

pub mod extent;
pub mod force;
pub mod quadtree;

pub(crate) mod lcg;

pub use force::*;
