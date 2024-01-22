//! Build script for the game binary.
//!
//! This script is run before the game binary is compiled. It is used to set the program icon and
//! to add build-time information to the binary.

extern crate embed_resource;

fn main() {
    // Add built.rs build-time information to the binary
    built::write_built_file().expect("Failed to acquire build-time information");
    // Set the program icon
    let Ok(target) = std::env::var("TARGET") else {
        println!("Failed to get target triple, cannot set program icon");
        return;
    };
    if target.contains("windows") {
        embed_resource::compile("./assets/icon.rc", embed_resource::NONE);
    }
}
