use bevy::{prelude::*, render::mesh::MeshVertexAttribute, gltf::GltfMesh};
use bevy_rapier3d::prelude::*;
use rand::Rng;

pub struct TreePlugin;

impl Plugin for TreePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup_tree)
			.add_systems(Update, spawn_leaves);
			// .add_systems(Update, (floor_hits, ball_land_event_handler))
			// .add_event::<BallLandEvent>();
	}
}

const FLYING_LEAVES_COUNT: u32 = 250;

fn setup_tree (
	mut commands: Commands,
	ass: Res<AssetServer>
){
    let leave = ass.load("assets.glb#Scene2");

	let tree: Handle<Mesh> = ass.load("assets.glb#Mesh8/Primitive0");
	let tree_mat: Handle<StandardMaterial> = ass.load("assets.glb#Material0");

    commands.spawn(PbrBundle {
		mesh: tree,
		material: tree_mat,
		..Default::default()
	}).insert(IsTree{has_leaves:false});

	// flying leaves
	for _i in 0..FLYING_LEAVES_COUNT {
		let x = rand::thread_rng().gen_range(-10..10) as f32;
		let y = rand::thread_rng().gen_range(0.3..10.0) as f32;
		let z = rand::thread_rng().gen_range(-10..10) as f32;

		let mut transform = Transform::from_xyz(x,y,z);
		transform.rotate(
			Quat::from_euler(EulerRot::XYZ, 
				rand::thread_rng().gen_range(-10..10) as f32,
				rand::thread_rng().gen_range(-10..10) as f32,
				rand::thread_rng().gen_range(-10..10) as f32
			)
		);

		commands.spawn(SceneBundle {
			scene: leave.clone(),
			transform: transform.with_scale(Vec3::splat(0.5)),
			..Default::default()
		});
	}

}

#[derive(Component)]
struct IsTree {
	has_leaves: bool
}

#[derive(Component)]
struct FoundTree {}

fn spawn_leaves(
	mut meshes: Query<(&Handle<Mesh>, &mut IsTree), With<IsTree>>,
	mesh_assets: ResMut<Assets<Mesh>>,
	ass: Res<AssetServer>,
    mut commands: Commands,
) {

	for (mesh_handle, mut tree) in meshes.iter_mut() {
		if tree.has_leaves {
			continue;
		}
		
		if let Some(mesh) = mesh_assets.get(mesh_handle) {

			let leaf = ass.load("assets.glb#Scene2");

			if let Some(vertices) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
				for v in vertices.as_float3() {
					for vv in v {

						if vv[1] < 2.0 {continue};

						let x = vv[0] + rand::thread_rng().gen_range(-0.15..0.15) as f32;
						let y = vv[1] + rand::thread_rng().gen_range(-0.15..0.15) as f32;
						let z = vv[2] + rand::thread_rng().gen_range(-0.15..0.15) as f32;

						let mut transform = Transform::from_xyz(x,y,z);
						transform.rotate(
							Quat::from_euler(EulerRot::XYZ, 
								rand::thread_rng().gen_range(-10..10) as f32,
								rand::thread_rng().gen_range(-10..10) as f32,
								rand::thread_rng().gen_range(-10..10) as f32
							)
						);
						commands.spawn(SceneBundle {
							scene: leaf.clone(),
							transform: transform.with_scale(Vec3::splat(0.5)),
							..Default::default()
						});
					}
 
				}
				tree.has_leaves = true;
			}
		};
    }
}