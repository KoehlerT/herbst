use bevy::prelude::*;
use super::*;


pub fn ball_land_event_handler 
(
	mut event_reader: EventReader<BallLandEvent>,
	mut commands: Commands
){
	for event in event_reader.read() {
		commands.entity(event.ball).despawn_recursive()
	}
}