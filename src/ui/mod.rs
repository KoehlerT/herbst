use bevy::prelude::*;
pub struct UiPlugin;

mod layout;
mod systems;

use crate::game::{Score, AppState};

#[derive(Component)]
pub struct ScoreMarker;


#[derive(Component)]
pub struct TimeMarker;


impl Plugin for UiPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(OnEnter(AppState::StartMenu), layout::create_startmenu_ui)
			.add_systems(OnExit(AppState::StartMenu), layout::hide_start_menu_ui)
			.add_systems(OnEnter(AppState::InGame), layout::create_game_ui)
			.add_systems(OnExit(AppState::InGame), layout::hide_game_ui)
			.add_systems(OnEnter(AppState::EndScreen), layout::create_endscreen_ui)
			.add_systems(OnExit(AppState::EndScreen), layout::hide_endscreen_ui)
			.add_systems(Update, systems::increase_score_event_handler.run_if(in_state(AppState::InGame)))
			.add_systems(Update, systems::update_time_event_handler.run_if(in_state(AppState::InGame)))
			.add_systems(Update, systems::start_button_system.run_if(in_state(AppState::StartMenu)))
			.add_systems(Update, systems::restart_button_system.run_if(in_state(AppState::EndScreen)));
		}
}
