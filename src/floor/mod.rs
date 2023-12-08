use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod hit_floor;
use hit_floor::*;
mod landings;
use landings::*;

pub struct FloorPlugin;

#[derive(Component)]
pub struct FloorMarker;

#[derive(Event)]
pub struct BallLandEvent {
	ball: Entity
}

impl Plugin for FloorPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup_floor)
			.add_systems(Update, (floor_hits, ball_land_event_handler))
			.add_event::<BallLandEvent>();
	}
}

fn setup_floor (
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
	commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(6.0).into()),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    }).insert((
		RigidBody::Fixed,
		Collider::cuboid(6.0, 6.0, 0.1),
		Friction::coefficient(0.7),
		ActiveEvents::COLLISION_EVENTS,
		FloorMarker
	));
}