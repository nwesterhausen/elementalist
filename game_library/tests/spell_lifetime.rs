use game_library::spells::SpellLifetime;

#[test]
fn spell_lifetime() {
    let mut spell_lifetime = SpellLifetime::new(10.0);
    let almost_eq = (spell_lifetime.max() - 10.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (spell_lifetime.remaining() - 10.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert!(!spell_lifetime.is_expired());

    spell_lifetime.update(2.);

    let almost_eq = (spell_lifetime.max() - 10.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (spell_lifetime.remaining() - 8.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert!(!spell_lifetime.is_expired());

    spell_lifetime.update(8.);

    let almost_eq = (spell_lifetime.max() - 10.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    let almost_eq = (spell_lifetime.remaining() - 0.0).abs() < f32::EPSILON;
    assert!(almost_eq);
    assert!(spell_lifetime.is_expired());
}
