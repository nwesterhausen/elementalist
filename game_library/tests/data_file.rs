use elementalist_game_library::data_loader::*;
use elementalist_game_library::enums::GameSystem;
use elementalist_game_library::{spells::Spell, Tileset};

#[test]
fn try_into() {
    let data_file = DataFile {
        header: DataFileHeader {
            system: GameSystem::Spell,
            ..DataFileHeader::default()
        },
        data: Spell::default(),
    };

    let spell: Result<Spell, ()> = data_file.try_into();
    assert!(spell.is_ok());

    let data_file = DataFile {
        header: DataFileHeader {
            system: GameSystem::Tileset,
            ..DataFileHeader::default()
        },
        data: Tileset::default(),
    };

    let tileset: Result<Tileset, ()> = data_file.try_into();
    assert!(tileset.is_ok());
}

#[test]
fn try_into_wrong() {
    let data_file = DataFile {
        header: DataFileHeader {
            system: GameSystem::Spell,
            ..DataFileHeader::default()
        },
        data: Spell::default(),
    };

    let tileset: Result<Tileset, ()> = data_file.try_into();
    assert!(tileset.is_err());
}
