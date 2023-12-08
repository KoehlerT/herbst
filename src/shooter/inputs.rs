use bevy::prelude::*;
use bevy::math::*;

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
	if keyboard_input.pressed(KeyCode::A) { input_dir.x = -1.;}
	// if keyboard_input.pressed(KeyCode::S) { input_dir.z = -1.;}
	if keyboard_input.pressed(KeyCode::D) { input_dir.x = 1.;}

	// let r = Vec3::abs(camera_transform.translation);
	// let alpha = arctan(camera_transform.translation.y / camera_transform.translation.x);
	// let move_dir = dir * input_dir.x; //Vec3::from((camera_transform.forward().x,0.,camera_transform.forward().z)) * input_dir.z + camera_transform.right() * input_dir.x;
	if input_dir.x != 0.0 {
		let mut copy = camera_transform.clone();
		copy.look_at(Vec3::ZERO, Vec3::Y);
		let dir : Vec3 = copy.left();
		let move_dir = dir * input_dir.x; 
		camera_transform.translation += move_dir * MOVEMENT_SPEED * time.delta().as_secs_f32();
		camera_transform.look_at(Vec3::ZERO, Vec3::Y);
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