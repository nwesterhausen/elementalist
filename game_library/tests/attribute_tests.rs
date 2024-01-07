use game_library::Attribute;

#[test]
fn new() {
    let attribute = Attribute::new(20_u32);

    assert_eq!(attribute.is_empty(), false);
    assert_eq!(attribute.is_full(), true);
    assert_eq!(attribute.remaining(), 1.);
    assert_eq!(attribute.percentage_remaining(), 100);
}

//Todo: add more tests
