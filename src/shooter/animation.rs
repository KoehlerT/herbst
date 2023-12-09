use bevy::prelude::*;
use super::*;

pub fn load_animations(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.insert_resource(Animations(vec![
		asset_server.load("assets.glb#Animation0"),
		asset_server.load("assets.glb#Animation1"),
		asset_server.load("assets.glb#Animation2"),
		asset_server.load("assets.glb#Animation3"),
		asset_server.load("assets.glb#Animation4"),
	]));
}

pub fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut players: Query<(&mut AnimationPlayer, &Parent), Added<AnimationPlayer>>,
) {
	let mut i = 0;
    for (mut player, _parent) in &mut players {
		player.play(animations.0[i].clone_weak());
		player.set_speed(2.);
		i = i+1;
    }
}

pub fn replay_animation(
	mut players: Query<&mut AnimationPlayer>,
) {
    for mut player in &mut players {
		player.replay();
    }
}