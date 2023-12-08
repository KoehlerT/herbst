use std::time::Duration;

use bevy::prelude::*;
pub struct ShooterPlugin;

mod camera;
mod inputs;
mod launcher;

#[derive(Event)]
pub struct TriggerShotEvent {
	direction: Transform,
	magnitude: f32
}
#[derive(Resource)]
pub struct CooldownTimer{
	timer: Timer
}

#[derive(Component)]
pub struct BallMarker;

impl Plugin for ShooterPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, camera::spawn_camera)
			.add_systems(Update, (inputs::keyboard_movements, inputs::shoot_trigger, ))
			.add_systems(Update, (inputs::keyboard_movements, inputs::shoot_trigger, launcher::launch_ball, tick_cooldown_timer))
			.add_event::<TriggerShotEvent>()
			.insert_resource(CooldownTimer {timer: Timer::new(Duration::from_secs_f32(0.5), TimerMode::Once)});
	}
}

fn tick_cooldown_timer(
	mut timer : ResMut<CooldownTimer>,
	time: Res<Time>
) {
	if !timer.timer.finished() {
		timer.timer.tick(time.delta());
	}
}