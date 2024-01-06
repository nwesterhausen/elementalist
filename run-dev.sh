#!/bin/sh
## Might simplify running with dynamic linking enabled, if hitting the
## up arrow (or calling history) doesn't do it for you.

cargo run -p elementalist --features bevy/dynamic_linking
