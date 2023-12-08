use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct TreePlugin;

impl Plugin for TreePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup_tree);
			// .add_systems(Update, (floor_hits, ball_land_event_handler))
			// .add_event::<BallLandEvent>();
	}
}

fn setup_tree (
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
	ass: Res<AssetServer>
){
    let tree = ass.load("assets.glb#Scene3");

    commands.spawn(SceneBundle {
        scene: tree,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });

}