use game_library::spells;

#[test]
fn spell_talisman_default() {
    let talisman = spells::talisman::SpellTalisman::default();
    assert_eq!(talisman.shaping(), spells::talisman::Shaping::Projectile);
    assert_eq!(talisman.behavior(), spells::talisman::Behavior::Damage);
    assert_eq!(talisman.tier(), spells::talisman::Tier::Mundane);
}

#[test]
fn spell_talisman_with_shaping() {
    let talisman =
        spells::talisman::SpellTalisman::default().with_shaping(spells::talisman::Shaping::Touch);
    assert_eq!(talisman.shaping(), spells::talisman::Shaping::Touch);
    assert_eq!(talisman.behavior(), spells::talisman::Behavior::Damage);
    assert_eq!(talisman.tier(), spells::talisman::Tier::Mundane);
}

#[test]
fn spell_talisman_with_behavior() {
    let talisman =
        spells::talisman::SpellTalisman::default().with_behavior(spells::talisman::Behavior::Heal);
    assert_eq!(talisman.shaping(), spells::talisman::Shaping::Projectile);
    assert_eq!(talisman.behavior(), spells::talisman::Behavior::Heal);
    assert_eq!(talisman.tier(), spells::talisman::Tier::Mundane);
}

#[test]
fn spell_talisman_with_tier() {
    let talisman =
        spells::talisman::SpellTalisman::default().with_tier(spells::talisman::Tier::Rare);
    assert_eq!(talisman.shaping(), spells::talisman::Shaping::Projectile);
    assert_eq!(talisman.behavior(), spells::talisman::Behavior::Damage);
    assert_eq!(talisman.tier(), spells::talisman::Tier::Rare);
}

#[test]
fn spell_talisman_new() {
    let talisman = spells::talisman::SpellTalisman::new(
        spells::talisman::Tier::Epic,
        spells::talisman::Shaping::AreaOfEffect,
        spells::talisman::Behavior::Debuff,
    );
    assert_eq!(talisman.shaping(), spells::talisman::Shaping::AreaOfEffect);
    assert_eq!(talisman.behavior(), spells::talisman::Behavior::Debuff);
    assert_eq!(talisman.tier(), spells::talisman::Tier::Epic);
}
