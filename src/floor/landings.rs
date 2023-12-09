use bevy::prelude::*;
use crate::game::Score;

use super::*;


pub fn ball_land_event_handler 
(
	mut event_reader: EventReader<BallLandEvent>,
	mut commands: Commands,
	mut score: ResMut<Score>
){
	for event in event_reader.read() {
		commands.entity(event.ball).despawn_recursive();
		score.value += 1;
	}
}