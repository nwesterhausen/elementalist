use game_library::Mana;

#[test]
fn mana() {
    let mut mana = Mana::new(19);

    assert_eq!(mana.value.is_empty(), false);
    assert_eq!(mana.value, 19);
    assert_eq!(mana.value.max, 19);

    mana.value.set(0_u32);

    assert_eq!(mana.value.is_empty(), true);
    assert_eq!(mana.value, 0);
    assert_eq!(mana.value.max, 19);
}

#[test]
fn mana_subtract() {
    let mut mana = Mana::new(19);

    assert_eq!(mana.value.is_empty(), false);
    assert_eq!(mana.value, 19);
    assert_eq!(mana.value.max, 19);

    mana.value -= 5;

    assert_eq!(mana.value.is_empty(), false);
    assert_eq!(mana.value, 14);
    assert_eq!(mana.value.max, 19);
}

#[test]
fn mana_add() {
    let mut mana = Mana::new(19);
    mana.value.set(14_u32);

    assert_eq!(mana.value.is_empty(), false);
    assert_eq!(mana.value, 14);
    assert_eq!(mana.value.max, 19);

    mana.value += 5;

    assert_eq!(mana.value.is_empty(), false);
    assert_eq!(mana.value, 19);
    assert_eq!(mana.value.max, 19);
}
