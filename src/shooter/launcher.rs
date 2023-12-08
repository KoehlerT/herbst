use bevy::prelude::*;
use crate::shooter::TriggerShotEvent;
use bevy_rapier3d::prelude::*;

pub fn launch_ball (
	mut event_reader : EventReader<TriggerShotEvent>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
	mut timer : ResMut<super::CooldownTimer>,
) {
	for event in event_reader.read() {
		if timer.timer.finished() {
			spawn_ball(&mut commands, &mut meshes, &mut materials, &event);
			timer.timer.reset();
		}
		
	}
}

fn spawn_ball(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
	event: &TriggerShotEvent
){
	let tf = event.direction.translation + (event.direction.forward()*1.2);
	commands.spawn(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 0.5, ..default() })),
		material: materials.add(Color::rgb_u8(124, 144, 255).into()),
		transform: event.direction.with_translation(tf),
		..default()
	}).insert((
		RigidBody::Dynamic,
		GravityScale(0.5),
		Velocity {
			linvel: event.direction.forward() * event.magnitude,
			angvel: Vec3::new(0.2, 0.0, 0.0),
		},
		Collider::ball(0.5)
	));
}