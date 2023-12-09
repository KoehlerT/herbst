use bevy::prelude::*;
pub struct UiPlugin;

mod layout;
mod systems;

use crate::game::Score;

#[derive(Component)]
pub struct ScoreMarker;


#[derive(Component)]
pub struct TimeMarker;


impl Plugin for UiPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, layout::create_ui)
			.add_systems(Update, systems::increase_score_event_handler)
			.add_systems(Update, systems::update_time_event_handler);
	}
}
