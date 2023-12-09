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
	round_time.value = 5.0;
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

fn generate_gameover_message(score: &Score) -> String {
	let percentage = score.value as f32 / score.leave_count as f32;
	if percentage >= 1. {"Wow - how did you do that?".into()}
	else if percentage >= 0.9 {"The winter is gonna be a breeze".into()}
	else if percentage >= 0.75 {"That's alright :-)".into()}
	else if percentage >= 0.5 {"Don't leaf me like this".into()}
	else if percentage >= 0.2 {"I'm trembling like aspen leaves".into()}
	else {"Oh no, that's gonna be cold!".into()}
}
