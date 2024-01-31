# Primal Realms

The main game loop revolves around going into and coming out of the primal realms.

## Lore

As a budding elementalist, you find out how to create and stabilize a portal to the Primal Realms: a collection of elemental planes. This will allow you to easily gather elemental essences. Elementalists are able to harness the elemental essence to develop their craft.

## Implementation Details

Ideas for the Primal Realms:

- Procedurally generated maps
- Initially no / very few monsters in the realm but more essence extracted will bring more monsters
- Can choose to "buff" the realm teleporting to for more essence generation but will be more dangerous
- The element of the realm can be influenced before entering
- Elements may not be one-to-one for skills. Thinking something like a metal realm which gives fire and earth essence.

What does this mean:

- many sprites/assets for different areas (trees, rocks, grass, etc)
- have to go from procedural generation to a map in bevy
- is it possible to save while in a realm, since the map is random? could generate from a seed but then how to save what has been done?
- can we load in objects to populate the realms from game_data?
