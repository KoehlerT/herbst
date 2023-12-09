use std::time::Duration;

use bevy::{prelude::*, ecs::system::SystemId};
pub struct ShooterPlugin;

mod camera;
mod inputs;
mod launcher;
mod animation;
use animation::*;

use crate::game::AppState;

#[derive(Event)]
pub struct TriggerShotEvent {
	direction: Transform,
	magnitude: f32
}
#[derive(Resource)]
pub struct CooldownTimer{
	timer: Timer
}

#[derive(Resource)]
pub struct Animations(Vec<Handle<AnimationClip>>);

#[derive(Resource)]
pub struct PlayAnimationSystem(SystemId);

#[derive(Component)]
pub struct BallMarker;

#[derive(Resource)]
pub struct ShooterPower(f32);

impl Plugin for ShooterPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, (camera::spawn_camera, load_animations))
			.add_systems(Update, ((inputs::keyboard_movements, inputs::shoot_trigger).run_if(in_state(AppState::InGame)), launcher::launch_ball, launcher::reload, tick_cooldown_timer, setup_scene_once_loaded))
			.add_event::<TriggerShotEvent>()
			.insert_resource(CooldownTimer {timer: Timer::new(Duration::from_secs_f32(3.), TimerMode::Once)})
			.insert_resource(ShooterPower(15.0));
		let id = app.world.register_system(animation::replay_animation);
		app.insert_resource(PlayAnimationSystem(id));
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