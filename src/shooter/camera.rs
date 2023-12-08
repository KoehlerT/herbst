use bevy::prelude::*;

pub fn spawn_camera(
	mut commands: Commands
) {
	// camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}