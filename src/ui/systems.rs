use bevy::prelude::*;

use crate::game::{Score, RoundTime, AppState};

use super::layout::{HOVERED_BUTTON, NORMAL_BUTTON};

pub fn start_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor
        ),
        (Changed<Interaction>, With<Button>),
    >,
	mut state: ResMut<NextState<AppState>>
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                state.set(AppState::InGame);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn restart_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor
        ),
        (Changed<Interaction>, With<Button>),
    >,
	mut state: ResMut<NextState<AppState>>
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                state.set(AppState::InGame);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}


pub fn increase_score_event_handler (
	score: Res<Score>,
	mut query: Query<&mut Text, With<super::ScoreMarker>>
) {
	if score.is_changed() {
		let mut text = query.single_mut();
		let v = score.value;
		text.sections[1].value = format!("{v:.2}");
	}
}

pub fn update_time_event_handler (
	time: Res<RoundTime>,
	mut query: Query<&mut Text, With<super::TimeMarker>>
) {
	if time.is_changed() {
		let mut text = query.single_mut();
		let mins = (time.value / 60.0) as i32;
		let secs = (time.value % 60.0) as i32;
		text.sections[1].value = format!("{mins:02}:{secs:02}");
	}
}