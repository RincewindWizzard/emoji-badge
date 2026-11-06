//! Treiber und Gestenlogik, unabhängig vom Chip (nur `embedded-hal`).
//! Alles hier ist auf dem Host testbar; die ESP32-Anbindung liegt in `firmware/`.
#![no_std]

pub mod board;
pub mod co5300;
pub mod cst9217;
pub mod gesture;
