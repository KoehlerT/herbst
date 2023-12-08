use bevy::{prelude::*, math::vec3};
use crate::shooter::TriggerShotEvent;
use bevy_rapier3d::prelude::*;

const BALL_RADIUS :f32 = 0.08;
const BALL_OFFSET : Vec3 = vec3(0., -0.2, -1.);
#[derive(Component)]
pub struct ReloadedBall;

pub fn reload (
	mut timer : ResMut<super::CooldownTimer>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
	transform: Query<Entity, With<Camera3d>>
) {
	if timer.timer.finished() && !timer.timer.paused(){
		timer.timer.pause();
		timer.timer.reset();
		spawn_ball(&mut commands, &mut meshes, &mut materials, transform.single());
	}
}

pub fn launch_ball (
	mut event_reader : EventReader<TriggerShotEvent>,
	mut commands: Commands,
	mut timer : ResMut<super::CooldownTimer>,
	play_animation: Res<super::PlayAnimationSystem>,
	mut ball_query: Query<(Entity, &mut GravityScale, &mut Transform), With<ReloadedBall>>
) {
	for event in event_reader.read() {
		if timer.timer.paused() {
			info!("Launch!");
			timer.timer.unpause();
			// spawn_ball(&mut commands, &mut meshes, &mut materials, &event);
			if let Ok((ball, mut gravity, mut transform)) = ball_query.get_single_mut() {
				gravity.0 = 1.;
				transform.translation = event.direction.translation + event.direction.up()*BALL_OFFSET.y + event.direction.back()*BALL_OFFSET.z;
				commands.entity(ball).insert((
					Velocity {
						linvel: event.direction.forward() * event.magnitude,
						angvel: Vec3::new(0.2, 0.0, 0.0),
					},
				));
				commands.entity(ball).remove::<Parent>();
				commands.entity(ball).remove::<ReloadedBall>();
				timer.timer.reset();
				commands.run_system(play_animation.0);
			}
			
		}
		
	}
}

fn spawn_ball(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
	parent: Entity
){
	info!("Spawn");
	let ball_id = commands.spawn(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::UVSphere { radius: BALL_RADIUS, ..default() })),
		material: materials.add(Color::rgb_u8(124, 144, 255).into()),
		transform: Transform {translation: BALL_OFFSET, ..default()},
		..default()
	}).insert((
		RigidBody::Dynamic,
		Collider::ball(BALL_RADIUS),
		super::BallMarker,
		GravityScale(0.),
		Restitution {
			coefficient: 0.7,
			combine_rule: CoefficientCombineRule::Average,
		},
		ReloadedBall
	)).id();
	commands.entity(parent).add_child(ball_id);
}