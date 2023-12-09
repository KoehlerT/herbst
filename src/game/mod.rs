use bevy::prelude::*;


#[derive(Resource)]
pub struct Score {
	pub value: i32
}
#[derive(Resource)]
pub struct RoundTime {
	pub value: f32
}

pub struct GameMangerPlugin;

impl Plugin for GameMangerPlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(RoundTime{value: 60.*2.})
			.insert_resource(Score{value: 0})
			.add_systems(Update, update_time);
	}
}


pub fn update_time
(
	// mut commands: Commands,
	// mut score: ResMut<Score>,
	mut round_time: ResMut<RoundTime>,
	time: Res<Time>,
){
	if round_time.value <= 0. {return}
	round_time.value -= time.delta_seconds();
}

