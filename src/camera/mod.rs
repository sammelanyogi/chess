
mod system;
use bevy::prelude::*;

use system::spawn_camera;

pub struct CameraPlugin;


impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}


