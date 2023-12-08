use bevy::prelude::*;
pub struct UiPlugin;

mod layout;
mod systems;

#[derive(Component)]
pub struct ScoreMarker;

#[derive(Resource)]
pub struct Score {
	pub value: i32
}

impl Plugin for UiPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, layout::create_ui)
			.add_systems(Update, systems::increase_score_event_handler)
			.insert_resource(Score{value: 0});
	}
}
