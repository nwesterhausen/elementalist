#!/bin/sh
## The recommended way to run your Bevy app from WSL is to cross-compile for Windows.

cargo build --target x86_64-pc-windows-msvc &&
cp target/x86_64-pc-windows-msvc/debug/elementalist.exe . &&
exec ./elementalist.exe "$@"
