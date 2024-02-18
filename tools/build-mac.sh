#!/bin/bash

# Setup the game assets
mkdir -p elementalist.app/Contents/MacOS
mkdir -p elementalist.app/Contents/Resources
# copy game assets
cp -r game/assets elementalist.app/Contents/MacOS/
cp -r game_data elementalist.app/Contents/MacOS/
# copy metadata
cp game_assets/Info.plist elementalist.app/Contents/
cp game_assets/Icon/icon.icns elementalist.app/Contents/Resources/

# compile the executables for each architecture
cargo build --release --target x86_64-apple-darwin # build for Intel
cargo build --release --target aarch64-apple-darwin # build for Apple Silicon


# combine the executables into a single file and put it in the bundle
lipo "target/x86_64-apple-darwin/release/elementalist" \
     "target/aarch64-apple-darwin/release/elementalist" \
     -create -output "elementalist.app/Contents/MacOS/elementalist"
