use game_library::SpellLifetime;

#[test]
fn spell_lifetime() {
    let mut spell_lifetime = SpellLifetime::new(10.0);

    assert_eq!(spell_lifetime.max, 10.0);
    assert_eq!(spell_lifetime.remaining, 10.0);
    assert_eq!(spell_lifetime.is_expired(), false);

    spell_lifetime.update(2.);

    assert_eq!(spell_lifetime.max, 10.0);
    assert_eq!(spell_lifetime.remaining, 8.);
    assert_eq!(spell_lifetime.is_expired(), false);

    spell_lifetime.update(8.);

    assert_eq!(spell_lifetime.max, 10.0);
    assert_eq!(spell_lifetime.remaining, 0.0);
    assert_eq!(spell_lifetime.is_expired(), true);
}
