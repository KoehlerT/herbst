use bevy::prelude::*;

pub const NORMAL_BUTTON: Color = Color::rgb(0.79, 0.3, 0.3);
pub const HOVERED_BUTTON: Color = Color::GRAY;

pub fn create_game_ui(
	mut commands: Commands
) {
	let entity = commands.spawn(NodeBundle {
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
				align_content: AlignContent::FlexEnd,
				padding: UiRect::all(Val::Px(15.0)),
				..default()
			},
			..default()
		}).with_children(|builder| {
			builder.spawn(TextBundle::from_sections([
				TextSection::new(
					"Time:  ".to_string(),
					TextStyle {
						font: Default::default(),
						font_size: 48.0,
						color: Color::WHITE,
				}),
				TextSection::new(
					"00:00".to_string(),
					TextStyle {
						font: Default::default(),
						font_size: 48.0,
						color: Color::rgb(0.79, 0.3, 0.3)
					})
				])
			).insert(super::TimeMarker);
		});
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
						color: Color::rgb(0.79, 0.3, 0.3)
					})
				])
			).insert(super::ScoreMarker);
		});
	}).id();

	commands.insert_resource(InGameUIData { entity });

}


#[derive(Resource)]
pub struct StartMenuData {
    entity: Entity,
}

#[derive(Resource)]
pub struct InGameUIData {
    entity: Entity,
}

#[derive(Resource)]
pub struct EndScreenData {
    entity: Entity,
}

pub fn create_startmenu_ui(
	mut commands: Commands
) {
	let entity = commands.spawn(NodeBundle {
		style: Style { 
			display: Display::Flex,
			justify_content: JustifyContent::Center,
			align_items: AlignItems::Center,
			padding: UiRect::all(Val::Px(50.0)),
			width: Val::Vw(100.0),
			height: Val::Vh(100.0),
			
			..default()
		},
		..default()
	}).with_children(|builder| {
		// Text Container
		builder.spawn(NodeBundle {
			style: Style {
				display: Display::Flex,
				flex_direction: FlexDirection::Column,
				justify_content: JustifyContent::SpaceBetween,
				// // align_content: AlignContent::FlexStart,
				// justify_content: JustifyContent::Center,
				height: Val::Percent(80.0),
				// width: Val::Auto,
				// width: Val::Percent(100.0),
				// height: Val::Percent(100.0),
				border: UiRect::all(Val::Px(4.0)),
				padding: UiRect::all(Val::Px(30.0)),
				..default()
			},
			border_color: BorderColor(Color::WHITE),
			background_color: Color::rgba(0.15, 0.15, 0.15, 0.85).into(),
			..default()
		}).with_children(|builder| {
			builder.spawn(TextBundle::from_sections([
				TextSection::new(
					"herbst".to_string(),
					TextStyle {
						font: Default::default(),
						font_size: 48.0,
						color: Color::rgb(0.79, 0.3, 0.3),
				}),
				TextSection::new(
					"\n\nOh no - look - the leaves are not moving anymore - the wind froze them!
					But this tree need to get rid of its leaves to survive the winter.
					Help him by shooting down as many leaves as possible with your snowball crossbow.
					Hurry, you only have 30 seconds!
				
					WASDQE to move, SPACE to shoot".to_string(),
					TextStyle {
						font: Default::default(),
						font_size: 40.0,
						color: Color::WHITE,
				})
			]));

			builder.spawn(ButtonBundle {
                    style: Style {
                        width: Val::Auto,
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(4.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::WHITE),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start the Game",
                        TextStyle {
							font: Default::default(),
                            // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
		});
	}).id();

	commands.insert_resource(StartMenuData { entity });
}


pub fn create_endscreen_ui(
	mut commands: Commands
) {
	let entity = commands.spawn(NodeBundle {
		style: Style { 
			display: Display::Flex,
			justify_content: JustifyContent::Center,
			align_items: AlignItems::Center,
			padding: UiRect::all(Val::Px(50.0)),
			width: Val::Vw(100.0),
			height: Val::Vh(100.0),
			
			..default()
		},
		..default()
	}).with_children(|builder| {
		// Text Container
		builder.spawn(NodeBundle {
			style: Style {
				display: Display::Flex,
				flex_direction: FlexDirection::Column,
				justify_content: JustifyContent::SpaceBetween,
				// // align_content: AlignContent::FlexStart,
				// justify_content: JustifyContent::Center,
				height: Val::Percent(80.0),
				// width: Val::Auto,
				// width: Val::Percent(100.0),
				// height: Val::Percent(100.0),
				border: UiRect::all(Val::Px(4.0)),
				padding: UiRect::all(Val::Px(30.0)),
				..default()
			},
			border_color: BorderColor(Color::WHITE),
			background_color: Color::rgba(0.15, 0.15, 0.15, 0.85).into(),
			..default()
		}).with_children(|builder| {
			builder.spawn(TextBundle::from_sections([
				TextSection::new(
					"Endscreen".to_string(),
					TextStyle {
						font: Default::default(),
						font_size: 48.0,
						color: Color::rgb(0.79, 0.3, 0.3),
				}),
				TextSection::new(
					"\n\nOh no - look - the leaves are not moving anymore - the wind froze them!
					But this tree need to get rid of its leaves to survive the winter.
					Help him by shooting down as many leaves as possible with your snowball crossbow.
					Hurry, you only have 30 seconds!
				
					WASDQE to move, SPACE to shoot".to_string(),
					TextStyle {
						font: Default::default(),
						font_size: 40.0,
						color: Color::WHITE,
				})
			]));

			builder.spawn(ButtonBundle {
                    style: Style {
                        width: Val::Auto,
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(4.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::WHITE),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play again",
                        TextStyle {
							font: Default::default(),
                            // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
		});
	}).id();

	commands.insert_resource(EndScreenData { entity });
}


pub fn hide_start_menu_ui(
	mut commands: Commands, 
	menu_data: Res<StartMenuData>
) {
    commands.entity(menu_data.entity).despawn_recursive();
}

pub fn hide_game_ui(
	mut commands: Commands, 
	ingame_ui_data: Res<InGameUIData>
) {
    commands.entity(ingame_ui_data.entity).despawn_recursive();
}

pub fn hide_endscreen_ui(
	mut commands: Commands, 
	endscreen_data: Res<EndScreenData>
) {
    commands.entity(endscreen_data.entity).despawn_recursive();
}