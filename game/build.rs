extern crate embed_resource;

fn main() {
    // Add built.rs build-time information to the binary
    built::write_built_file().expect("Failed to acquire build-time information");
    // Set the program icon
    let target = std::env::var("TARGET").unwrap();
    if target.contains("windows") {
        embed_resource::compile("./assets/icon.rc", embed_resource::NONE);
    }
}
