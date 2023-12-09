
use bevy::{prelude::*, window::PresentMode, asset::AssetMetaCheck};
use bevy_rapier3d::prelude::*;

mod shooter;
mod floor;
mod ui;
mod tree;
mod game;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::rgb(0.21, 0.26, 0.32)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync, // Reduces input lag.
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
		.add_plugins((shooter::ShooterPlugin, floor::FloorPlugin, ui::UiPlugin, tree::TreePlugin))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(game::GameMangerPlugin)
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands
) {
    
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 2500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.06,
    });
}