use bevy::prelude::*;
pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle {
        camera: Camera {
            hdr: true, // 1. HDR is required for bloom
            ..default()
        },
        ..default()
    },));
}
