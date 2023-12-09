use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
	#[default]
    StartMenu,
    InGame,
	EndScreen
}

#[derive(Resource)]
pub struct Score {
	pub value: i32,
	pub leave_count: i32
}
#[derive(Resource)]
pub struct RoundTime {
	pub value: f32
}

pub struct GameMangerPlugin;

impl Plugin for GameMangerPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(OnEnter(AppState::InGame), reset_state)
			.insert_resource(RoundTime{value: 10.})
			.insert_resource(Score{value: 0, leave_count: 0})
			.add_systems(Update, update_time.run_if(in_state(AppState::InGame)))
			.add_state::<AppState>();
	}
}


pub fn reset_state(
	mut round_time: ResMut<RoundTime>,
	mut score: ResMut<Score>,
){
	round_time.value = 40.0;
	score.value = 0;
}


pub fn update_time(
	mut round_time: ResMut<RoundTime>,
	mut state: ResMut<NextState<AppState>>,
	time: Res<Time>,
){
	if round_time.value <= 0. {
		state.set(AppState::EndScreen);
		return
	}
	round_time.value -= time.delta_seconds();
}