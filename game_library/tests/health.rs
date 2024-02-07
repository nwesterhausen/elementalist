use game_library::Health;

#[test]
fn health() {
    let mut health = Health::new(19);

    assert!(!health.is_dead());
    assert_eq!(health.value, 19);
    assert_eq!(health.value.max, 19);

    health.kill();

    assert!(health.is_dead());
    assert_eq!(health.value, 0);
    assert_eq!(health.value.max, 19);
}

#[test]
fn health_subtract() {
    let mut health = Health::new(19);

    assert!(!health.is_dead());
    assert_eq!(health.value, 19);
    assert_eq!(health.value.max, 19);

    health.value -= 5;

    assert!(!health.is_dead());
    assert_eq!(health.value, 14);
    assert_eq!(health.value.max, 19);
}

#[test]
fn health_add() {
    let mut health = Health::new(19);
    health.value.set(14_u32);

    assert!(!health.is_dead());
    assert_eq!(health.value, 14);
    assert_eq!(health.value.max, 19);

    health.value += 5;

    assert!(!health.is_dead());
    assert_eq!(health.value, 19);
    assert_eq!(health.value.max, 19);
}
