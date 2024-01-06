//! # Elementalist
//!
//! A rogue-lite game where you play as an elementalist.

#![warn(
    missing_docs,
    unreachable_code,
    unreachable_patterns,
    clippy::unwrap_used,
    clippy::expect_used
)]
#![deny(unsafe_code)]

use bevy::prelude::*;

fn main() {
    App::new().add_systems(Update, hello_world).run();
}

fn hello_world() {
    println!("hello world!");
}
