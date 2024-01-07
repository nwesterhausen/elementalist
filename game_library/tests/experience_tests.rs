use game_library::Xp;

#[test]
fn test_xp_required() {
    let xp = Xp::default();

    assert_eq!(xp.xp_required(1), 10); // 10 xp to level 2
    assert_eq!(xp.xp_required(2), 10);
    assert_eq!(xp.xp_required(3), 10);
    assert_eq!(xp.xp_required(4), 11);
    assert_eq!(xp.xp_required(5), 13);
    assert_eq!(xp.xp_required(6), 15);
}

#[test]
fn total_xp_to_next_level() {
    let mut xp = Xp::default();

    assert_eq!(xp.total_xp_to_next_level(), 10);

    xp += 5;

    assert_eq!(xp.total_xp_to_next_level(), 10);
}

#[test]
fn remaining_xp_to_next_level() {
    let mut xp = Xp::default();

    assert_eq!(xp.remaining_xp_to_next_level(), 10);

    xp.value = 5;
    assert_eq!(xp.remaining_xp_to_next_level(), 5);

    xp.value = 10;
    assert_eq!(xp.remaining_xp_to_next_level(), 0);

    xp.value = 11;
    assert_eq!(xp.remaining_xp_to_next_level(), 0);
}

#[test]
fn next_level_progress() {
    let mut xp = Xp::default();

    assert_eq!(xp.next_level_progress(), 0.);

    xp.value = 5;
    assert_eq!(xp.next_level_progress(), 0.5);

    xp.value = 10;
    assert_eq!(xp.next_level_progress(), 1.);

    xp.value = 11;
    assert_eq!(xp.next_level_progress(), 1.);
}

#[test]
fn xp_remaining_using_add() {
    let mut xp = Xp::default();

    assert_eq!(xp.xp_required(1), 10); // 10 xp to level 2
    assert_eq!(xp.remaining_xp_to_next_level(), 10);

    xp += 5;

    assert_eq!(xp.remaining_xp_to_next_level(), 5);

    xp += 5;

    assert_eq!(xp.remaining_xp_to_next_level(), 0);

    xp += 1;

    assert_eq!(xp.remaining_xp_to_next_level(), 0);
    assert_eq!(xp.total_xp, 11);
}

#[test]
fn level_up() {
    let mut xp = Xp::default();

    assert_eq!(xp.current_level, 1);
    assert_eq!(xp.remaining_xp_to_next_level(), 10);

    xp += 10;
    xp.level_up();

    assert_eq!(xp.current_level, 2);
    assert_eq!(xp.remaining_xp_to_next_level(), 10);
}

#[test]
fn can_level_up() {
    let mut xp = Xp::default();

    assert_eq!(xp.can_level_up(), false);

    xp += 10;

    assert_eq!(xp.can_level_up(), true);
}

#[test]
fn level_up_with_overflow() {
    let mut xp = Xp::default();

    assert_eq!(xp.current_level, 1);
    assert_eq!(xp.remaining_xp_to_next_level(), 10);

    xp += 100;

    assert_eq!(xp.remaining_xp_to_next_level(), 0);

    xp.level_up();

    assert_eq!(xp.current_level, 2);
    assert_eq!(xp.remaining_xp_to_next_level(), 0);
}

#[test]
fn level_up_insufficient_xp() {
    let mut xp = Xp::default();

    assert_eq!(xp.current_level, 1);
    assert_eq!(xp.remaining_xp_to_next_level(), 10);

    xp += 5;
    xp.level_up();

    assert_eq!(xp.current_level, 1);
    assert_eq!(xp.remaining_xp_to_next_level(), 5);
}

#[test]
fn arbitrary_level_xp_requirements() {
    let xp = Xp::default();

    assert_eq!(xp.xp_required(1), 10); // 10 xp to level 2
    assert_eq!(xp.xp_required(2), 10);

    assert_eq!(xp.xp_required(10), 26); // 26 xp to level 11
    assert_eq!(xp.xp_required(11), 30); // 30 xp to level 12

    assert_eq!(xp.xp_required(20), 82); // 82 xp to level 21
    assert_eq!(xp.xp_required(21), 90); // 90 xp to level 22

    assert_eq!(xp.xp_required(30), 178); // 178 xp to level 31
    assert_eq!(xp.xp_required(31), 190); // 190 xp to level 32

    assert_eq!(xp.xp_required(40), 314);
    assert_eq!(xp.xp_required(41), 330);

    assert_eq!(xp.xp_required(50), 490);
    assert_eq!(xp.xp_required(51), 510);
}
