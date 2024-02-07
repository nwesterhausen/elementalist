use game_library::CameraScaleLevel;

#[test]
fn value() {
    let level = CameraScaleLevel::default();
    let almost_eq = (level.value() - 0.3).abs() < f32::EPSILON;
    assert!(almost_eq);
}

#[test]
fn zoom_out() {
    let mut level = CameraScaleLevel::default();
    let almost_eq = (level.value() - 0.3).abs() < f32::EPSILON;
    assert!(almost_eq);

    level.zoom_out();
    let almost_eq = (level.value() - 0.4).abs() < f32::EPSILON;
    assert!(almost_eq);
}

#[test]
fn zoom_in() {
    let mut level = CameraScaleLevel::default();
    let almost_eq = (level.value() - 0.3).abs() < f32::EPSILON;
    assert!(almost_eq);

    level.zoom_in();
    let almost_eq = (level.value() - 0.2).abs() < f32::EPSILON;
    assert!(almost_eq);
}

#[test]
fn zoom_in_min() {
    let mut level = CameraScaleLevel::default();
    let almost_eq = (level.value() - 0.3).abs() < f32::EPSILON;
    assert!(almost_eq);

    level.zoom_in();
    level.zoom_in();
    level.zoom_in();
    level.zoom_in();
    let almost_eq = (level.value() - CameraScaleLevel::LEVELS[0]).abs() < f32::EPSILON;
    assert!(almost_eq);
}

#[test]
fn zoom_out_max() {
    let mut level = CameraScaleLevel::default();
    let almost_eq = (level.value() - 0.3).abs() < f32::EPSILON;
    assert!(almost_eq);

    level.zoom_out();
    level.zoom_out();
    level.zoom_out();
    level.zoom_out();
    let almost_eq = (level.value() - CameraScaleLevel::LEVELS[CameraScaleLevel::LEVELS.len() - 1])
        .abs()
        < f32::EPSILON;
    assert!(almost_eq);
}

#[test]
fn implicit_from_f32() {
    let level: CameraScaleLevel = 0.5.into();
    let almost_eq = (level.value() - 0.5).abs() < f32::EPSILON;
    assert!(almost_eq);
}

#[test]
fn implicit_into_f32() {
    let level = CameraScaleLevel::default();

    let value: f32 = level.into();
    let almost_eq = (value - 0.3).abs() < f32::EPSILON;
    assert!(almost_eq);
}
