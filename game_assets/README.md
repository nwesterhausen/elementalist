# Game Assets

This directory has the raw graphics used in the game.

The graphics are put onto sprite sheets and then saved to the [assets](../game/assets) directory for use in the game.

## Tools Used

For working on the graphics I used the following tools:

- [Tiled](https://thorbjorn.itch.io/tiled): making the tilemaps/sprite sheets
- [Resprite](https://resprite.fengeon.com/): doing most of the pixel art editing on my tablet
- [Aseprite](https://www.aseprite.org/): doing any pixel art editing on the computer as needed

I save from Resprite on my tablet to a cloud storage and then copy them into this directory. Resprite lets you save 
as aseprite files which is nice. I can bulk export and it can all get uploaded and copied over in very few steps.
The way Tiled maps the images for the tile maps is dynamic -- that is, it loads it from the disk again when you open
the file. So if I make a change to anything, I can just export, reopen, resave. No re-creating or adding the image to
the tilemap again.

The icons were created using the [tauri icon](https://tauri.app/v1/guides/features/icons/) command. I wanted to do it 
on my own machine and not use a website to do the conversion. There's probably a simpler way but I was already familiar
with how tauri did things.
