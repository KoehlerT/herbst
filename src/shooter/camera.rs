use bevy::prelude::*;

pub fn spawn_camera(
	mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let cb: Handle<Scene> = asset_server.load("assets.glb#Scene1");
	// camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::new(0., 2., 0.), Vec3::Y),
        ..default()
    }).with_children(|builder| {
        builder.spawn(SceneBundle {
            scene: cb,
            transform: Transform {
                translation: Vec3::from((0.,-0.3,-1.4)),
                scale: Vec3::splat(0.6),
                rotation: Quat::from_euler(EulerRot::XYZ, 0.,3.1418, 0.),
                ..default()
            },
            ..default()
        });
    }).insert(VisibilityBundle::default());
}