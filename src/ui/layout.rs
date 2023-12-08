use bevy::prelude::*;

pub fn create_ui(
	mut commands: Commands
) {
	commands.spawn(NodeBundle {
		style: Style { 
			display: Display::Grid,
			grid_template_rows: vec![GridTrack::px(50.0), GridTrack::auto()],
			..default()
		},
		..default()
	}).with_children(|builder| {
		// Text Container
		builder.spawn(NodeBundle {
			style: Style {
				display: Display::Flex,
				flex_direction: FlexDirection::Column,
				padding: UiRect::all(Val::Px(15.0)),
				..default()
			},
			..default()
		}).with_children(|builder| {
			builder.spawn(TextBundle::from_sections([
				TextSection::new(
					"Score: ".to_string(),
					TextStyle {
						font: Default::default(),
						font_size: 48.0,
						color: Color::WHITE,
				}),
				TextSection::new(
					"000".to_string(),
					TextStyle {
						font: Default::default(),
						font_size: 48.0,
						color: Color::GOLD,
					})
				])
			).insert(super::ScoreMarker);
		});
	});
}