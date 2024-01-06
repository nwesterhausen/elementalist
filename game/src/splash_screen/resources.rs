use bevy::prelude::*;

// Timer which is used to control the splash screen duration
#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(pub Timer);
