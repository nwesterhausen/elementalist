use game_library::Volume;

#[test]
fn mute() {
    let mut volume = Volume::from(19);

    assert_eq!(volume.is_muted(), false);
    assert_eq!(volume.volume::<u32>(), 19);

    volume.mute();

    assert_eq!(volume.is_muted(), true);
    assert_eq!(volume.volume::<u32>(), 0);
}

#[test]
fn unmute() {
    let mut volume = Volume {
        value: 19,
        muted: true,
    };

    assert_eq!(volume.is_muted(), true);
    assert_eq!(volume.volume::<u32>(), 0);

    volume.unmute();

    assert_eq!(volume.is_muted(), false);
    assert_eq!(volume.volume::<u32>(), 19);
}

#[test]
fn raw_volume() {
    let volume = Volume {
        value: 19,
        muted: true,
    };

    assert_eq!(volume.raw_volume::<u32>(), 19);
    assert_eq!(volume.volume::<u32>(), 0);
}

#[test]
fn conversion_over_max_u8() {
    let volume = Volume::from(101u8);

    assert_eq!(volume.raw_volume::<u32>(), 100);
}

#[test]
fn conversion_over_max_u16() {
    let volume = Volume::from(10100u16);

    assert_eq!(volume.raw_volume::<u32>(), 100);
}

#[test]
fn conversion_over_max_u32() {
    let volume = Volume::from(1010000u32);

    assert_eq!(volume.raw_volume::<u32>(), 100);
}

#[test]
fn conversion_over_max_u64() {
    let volume = Volume::from(10100000000u64);

    assert_eq!(volume.raw_volume::<u32>(), 100);
}

#[test]
fn conversion_over_max_i8() {
    let volume = Volume::from(101i8);

    assert_eq!(volume.raw_volume::<u32>(), 100);
}

#[test]
fn conversion_over_max_i16() {
    let volume = Volume::from(10100i16);

    assert_eq!(volume.raw_volume::<u32>(), 100);
}

#[test]
fn conversion_over_max_i32() {
    let volume = Volume::from(1010000i32);

    assert_eq!(volume.raw_volume::<u32>(), 100);
}

#[test]
fn conversion_over_max_i64() {
    let volume = Volume::from(10100000000i64);

    assert_eq!(volume.raw_volume::<u32>(), 100);
}

#[test]
fn conversion_over_max_f32() {
    let volume = Volume::from(1.01f32);

    assert_eq!(volume.raw_volume::<u32>(), 100);
}

#[test]
fn conversion_over_max_f64() {
    let volume = Volume::from(1.01f64);

    assert_eq!(volume.raw_volume::<u32>(), 100);
}

#[test]
fn conversion_under_min_i8() {
    let volume = Volume::from(-101i8);

    assert_eq!(volume.raw_volume::<u32>(), 0);
}

#[test]
fn conversion_under_min_i16() {
    let volume = Volume::from(-10100i16);

    assert_eq!(volume.raw_volume::<u32>(), 0);
}

#[test]
fn conversion_under_min_i32() {
    let volume = Volume::from(-1010000i32);

    assert_eq!(volume.raw_volume::<u32>(), 0);
}

#[test]
fn conversion_under_min_i64() {
    let volume = Volume::from(-10100000000i64);

    assert_eq!(volume.raw_volume::<u32>(), 0);
}

#[test]
fn conversion_under_min_f32() {
    let volume = Volume::from(-1.01f32);

    assert_eq!(volume.raw_volume::<u32>(), 0);
}

#[test]
fn conversion_under_min_f64() {
    let volume = Volume::from(-1.01f64);

    assert_eq!(volume.raw_volume::<u32>(), 0);
}

#[test]
fn implicit_conversion_u8() {
    let volume: Volume = 32u8.into();

    assert_eq!(volume.raw_volume::<u32>(), 32);
}

#[test]
fn implicit_conversion_u16() {
    let volume: Volume = 32u16.into();

    assert_eq!(volume.raw_volume::<u32>(), 32);
}

#[test]
fn implicit_conversion_u32() {
    let volume: Volume = 32u32.into();

    assert_eq!(volume.raw_volume::<u32>(), 32);
}

#[test]
fn implicit_conversion_u64() {
    let volume: Volume = 32u64.into();

    assert_eq!(volume.raw_volume::<u32>(), 32);
}

#[test]
fn implicit_conversion_i8() {
    let volume: Volume = 32i8.into();

    assert_eq!(volume.raw_volume::<u32>(), 32);
}

#[test]
fn implicit_conversion_i16() {
    let volume: Volume = 32i16.into();

    assert_eq!(volume.raw_volume::<u32>(), 32);
}

#[test]
fn implicit_conversion_i32() {
    let volume: Volume = 32i32.into();

    assert_eq!(volume.raw_volume::<u32>(), 32);
}

#[test]
fn implicit_conversion_i64() {
    let volume: Volume = 32i64.into();

    assert_eq!(volume.raw_volume::<u32>(), 32);
}

#[test]
fn implicit_conversion_f32() {
    let volume: Volume = 0.32f32.into();

    assert_eq!(volume.raw_volume::<u32>(), 32);
}

#[test]
fn implicit_conversion_f64() {
    let volume: Volume = 0.32f64.into();

    assert_eq!(volume.raw_volume::<u32>(), 32);
}
