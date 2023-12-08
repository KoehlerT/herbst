use bevy::prelude::*;
pub struct ShooterPlugin;

mod camera;
mod inputs;

#[derive(Event)]
pub struct TriggerShotEvent;

impl Plugin for ShooterPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, camera::spawn_camera)
			.add_systems(Update, inputs::keyboard_movements)
			.add_event::<TriggerShotEvent>();
	}
}