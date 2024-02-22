# Elementalist - Design Document

## Table of Contents

- [Elementalist - Design Document](#elementalist---design-document)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
    - [Game Summary Pitch](#game-summary-pitch)
    - [Inspiration](#inspiration)
    - [Player Experience](#player-experience)
    - [Platform](#platform)
    - [Development Software](#development-software)
    - [Genre](#genre)
    - [Target Audience](#target-audience)
  - [Concept](#concept)
    - [Gameplay Overview](#gameplay-overview)
    - [Primary Mechanics](#primary-mechanics)
    - [Secondary Mechanics](#secondary-mechanics)
  - [Art](#art)
    - [Design Limitations](#design-limitations)
    - [Design Goals](#design-goals)
  - [Sound](#sound)
    - [Music](#music)
    - [Sound Effects](#sound-effects)
  - [Game Experience](#game-experience)
    - [UI](#ui)
    - [Controls](#controls)

## Introduction

### Game Summary Pitch

Elementalist is a top-down resource gathering game where the player travels to different elemental realms, choosing spells to take with them in order to gather elements and increase their power. The player encounters increasing dangers as they go to more advanced realms and their power grows alongside it.

### Inspiration

- Vampire Survivors

The core of the genre (maybe not the first, but the one remembered). Start with barely anything and slowly unlock more stuff to play with, having difficulty ramp up all the while.

- Death Must Die

This is in the same genre, but what would be great inspiration for Elementalist is how good casting spells felt in it.

- Noita

Creating custom spells and specialized wands. Being able to interact completely with the environment.

### Player Experience

1. Player starts a new game. They have only a couple choices for what spells to start out with and then are left to their own devices for advancement paths.
2. Player travels to an elemental realm and breaks down the environment there to gather elemental essence, which flow back to the player when their avatar expires in the realm.
3. While in the realm, players can unlock upgrades for their spell casting and avatar stats, but these are not permanent.
4. Back in their research laboratory, the player can use elemental essence to upgrade spells, unlock new skill trees, and fuse gems to guide where the portal will take them.
5. Player levels up their skills by casting spells of a particular skill tree, and then as those level up there are many passive perks that are active regardless the spells chosen for a trip.
6. Goal is for the player to finish researching all the elements and be a master of them all.

The primary drives for playing are

- It's never the same map; the realms (even ones of the same element) have randomly generated terrain aligned to the elements.
- Becoming stronger and then overpowered -- or falling due to hubris
- Unlocking all the skills, and trying out different spell combinations

### Platform

The game is available for Linux, Windows and MacOS (Intel and Apple Silicon).

### Development Software

- **Engine**: Bevy
- **Graphics**: Aseprite
- **Music**: TBD
- *Sound Effects*: TBD

### Genre

Action Roguelike, RPG, Bullet Hell

### Target Audience

Players who like magic, coming up with optimal builds, being rewarded for causing havoc, feeling good about advancing their character

No gore or other excessive violence. Age would be teenagers and older?

## Concept

### Gameplay Overview

After starting a new game, the player can see an interface representing the research laboratory. They can optionally read a short lore dump explaining the setting or skip and just pick a starting spell and skill tree. After that they are guided to activate the Portal to the Elemental Realms and do a first run, where their avatar in the realm gather essence and the core loop is introduced (pick a loadout -> enter a realm -> return with new resources -> unlock/upgrade things).

### Primary Mechanics

The primary mechanics are choosing spells and then using them in the realm. The player has two "primary" spell slots. One is a cantrip slot and the other is a casted slot. Cantrips are free and tend to be the "basic" attack used in the realm. The casted slots cost mana (which is very limited, the player starts with 4 mana) but have more devastating effects.

As the player's avatar is casting spells in the realm, objects (trees, rocks, ruins of buildings, etc.) can be destroyed and some of the elemental essence that made them up is collected. That essence will come back to the player after their avatar expires in the realm.

### Secondary Mechanics

Essence can be crafted into elemental gems which can be used to attune the Portal to the Elemental Realms to go to different elemental realms.

Essence can be condensed and researched to learn new spells.

Players earn experience in skills by casting spells that belong to that skill tree. After getting a level a perk can be unlocked in the skill perk tree.

Later on, players can choose a third spell slot for a defensive spell. These cost mana but do things to more drastically interact with the environment, like placing temporary walls or other hazards for the beasts and monsters that are natural denizens of the realms.

While in the realms, players may discover an ultimate spell slot. It can be attuned with some essence and become a very strong spell (costing mana). After the avatar expires, so does the ultimate spell slot.

## Art

### Design Limitations

color palette, sprite size

### Design Goals

color tie-in for elements?

## Sound

### Music

info about music?

### Sound Effects

info for sound effects?

## Game Experience

### UI

The UI is using some fonts to evoke ancient engraving for many display elements. For the player's spellbook, skills, and other notes a handwriting font is used. It's optional for the player to change the fonts used to be easier to read if desired.

### Controls

Can use keyboard & mouse or gamepad. The player can move, cast a primary, secondary, defensive or ultimate spell, and pause the game (while in a realm).
