use game_library::CameraScaleLevel;

#[test]
fn value() {
    let level = CameraScaleLevel::default();

    assert_eq!(level.value(), 0.3);
}

#[test]
fn zoom_out() {
    let mut level = CameraScaleLevel::default();

    assert_eq!(level.value(), 0.3);

    level.zoom_out();

    assert_eq!(level.value(), 0.5);
}

#[test]
fn zoom_in() {
    let mut level = CameraScaleLevel::default();

    assert_eq!(level.value(), 0.3);

    level.zoom_in();

    assert_eq!(level.value(), 0.25);
}

#[test]
fn zoom_in_min() {
    let mut level = CameraScaleLevel::default();

    assert_eq!(level.value(), 0.3);

    level.zoom_in();
    level.zoom_in();
    level.zoom_in();
    level.zoom_in();

    assert_eq!(level.value(), CameraScaleLevel::LEVELS[0]);
}

#[test]
fn zoom_out_max() {
    let mut level = CameraScaleLevel::default();

    assert_eq!(level.value(), 0.3);

    level.zoom_out();
    level.zoom_out();
    level.zoom_out();
    level.zoom_out();

    assert_eq!(
        level.value(),
        CameraScaleLevel::LEVELS[CameraScaleLevel::LEVELS.len() - 1]
    );
}

#[test]
fn implicit_from_f32() {
    let level: CameraScaleLevel = 0.5.into();

    assert_eq!(level.value(), 0.5);
}

#[test]
fn implicit_into_f32() {
    let level = CameraScaleLevel::default();

    let value: f32 = level.into();

    assert_eq!(value, 0.3);
}
