use std::f64::consts::PI;

use bevy::prelude::*;
use bevy::math::*;

use crate::shooter::TriggerShotEvent;

const ROTATE_SPEED : f32 = 0.5;
const MOVEMENT_SPEED : f32 = 5.0;
// Q Yaw left
// E Yaw right
// W Pitch up
// S Pitch down
// A Move left
// D Move right
pub fn keyboard_movements( 
	keyboard_input: Res<Input<KeyCode>>,
	time: Res<Time>,
	mut camera_transform: Query<&mut Transform, With<Camera3d>>,
) {
	let mut input_dir = Vec3::ZERO;
	let mut camera_transform = camera_transform.single_mut();

	// if keyboard_input.pressed(KeyCode::W) { input_dir.z = 1.;}
	if keyboard_input.pressed(KeyCode::D) { input_dir.x = 1.;}
	// if keyboard_input.pressed(KeyCode::S) { input_dir.z = -1.;}
	if keyboard_input.pressed(KeyCode::A) { input_dir.x = -1.;}

	if input_dir.x != 0.0 {
		camera_transform.rotate_around(Vec3::new(0., 0., 0.), Quat::from_rotation_y(input_dir.x*0.01));
	}

	let rot_amount = ROTATE_SPEED * time.delta_seconds();
	if keyboard_input.pressed(KeyCode::Q) {
		camera_transform.rotate_y(rot_amount);
	}
	if keyboard_input.pressed(KeyCode::E) {
		camera_transform.rotate_y(-rot_amount);
	}
	if keyboard_input.pressed(KeyCode::W) {
		let r = camera_transform.right();
		camera_transform.rotate_axis(r, rot_amount);

	}
	if keyboard_input.pressed(KeyCode::S) {
		let r = camera_transform.right();
		camera_transform.rotate_axis(r, -rot_amount);
	}
}

pub fn shoot_trigger (
	keyboard_input: Res<Input<KeyCode>>,
	mut event_writer: EventWriter<TriggerShotEvent>,
	camera_transform: Query<&Transform, With<Camera3d>>
){
	let camera_transform = camera_transform.single();
	if keyboard_input.pressed(KeyCode::Space) {
		event_writer.send(TriggerShotEvent {
			direction: camera_transform.clone(),
			magnitude: 15.0
		});
	}
}