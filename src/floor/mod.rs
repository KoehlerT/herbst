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
        mesh: meshes.add(shape::Cylinder{
            radius: 5.0,
            height: 0.2,
            resolution: 64,
            segments: 1,
        }.into()),
        material: materials.add(Color::WHITE.into()),
        ..default()
    }).insert((
		RigidBody::Fixed,
		Collider::cuboid(5.0, 0.2, 5.0),
		Friction::coefficient(0.7),
		ActiveEvents::COLLISION_EVENTS,
		FloorMarker
	));
}