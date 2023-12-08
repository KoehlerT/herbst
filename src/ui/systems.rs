use bevy::prelude::*;

use super::Score;

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