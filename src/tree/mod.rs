use bevy::{prelude::*, render::mesh::MeshVertexAttribute, gltf::GltfMesh, utils::tracing::field::Empty};
use bevy_rapier3d::prelude::*;
use rand::Rng;

pub struct TreePlugin;

impl Plugin for TreePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup_tree)
			.add_systems(Startup, spawn_flying_leaves)
			.add_systems(Update, (spawn_leaves, update_flying_leaves, /*add_leaf_colliders,*/ handle_leaf_collisions));
			// .add_systems(Update, (floor_hits, ball_land_event_handler))
			// .add_event::<BallLandEvent>();
	}
}

const FLYING_LEAVES_COUNT: u32 = 350;

#[derive(Resource)]
struct LeafMeshHandle(Handle<Mesh>);

fn setup_tree (
	mut commands: Commands,
	ass: Res<AssetServer>
){

	let tree: Handle<Mesh> = ass.load("assets.glb#Mesh8/Primitive0");
	let tree_mat: Handle<StandardMaterial> = ass.load("assets.glb#Material0");
	let leaf_handle: Handle<Mesh> = ass.load("assets.glb#Mesh7/Primitive0");
	info!("leaf handle {:?}", leaf_handle);
	commands.insert_resource(LeafMeshHandle(leaf_handle));

    commands.spawn(PbrBundle {
		mesh: tree,
		material: tree_mat,
		..Default::default()
	}).insert(Tree{has_leaves:false});

}

#[derive(Component)]
struct Tree {
	has_leaves: bool
}

#[derive(Component)]
struct FlyingLeave;

fn spawn_flying_leaves(
	mut commands: Commands,
	ass: Res<AssetServer>
) {
	let leave = ass.load("assets.glb#Scene2");

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
			transform: transform,
			..Default::default()
		}).insert(FlyingLeave);
	}
}


fn update_flying_leaves(
	mut transforms: Query<&mut Transform, With<FlyingLeave>>
) {
	for mut transform in transforms.iter_mut() {
		transform.translate_around(Vec3::new(0., 0., 0.), Quat::from_rotation_y(0.01));
		transform.rotate_local_x(0.01);
		transform.rotate_local_y(0.02);
	}
}

#[derive(Component)]
struct LeafMarker;

fn spawn_leaves(
	mut meshes: Query<(&Handle<Mesh>, &mut Tree), With<Tree>>,
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
						spawn_leaf(transform, &leaf, &mut commands);
					}
 
				}
				tree.has_leaves = true;
			}
		};
    }
}

fn spawn_leaf(transform: Transform, handle: &Handle<Scene>, commands: &mut Commands) {
	commands.spawn(SceneBundle {
		scene: handle.clone(),
		transform: transform,
		..Default::default()
	});
}

fn add_leaf_colliders(
	leaves: Query<(Entity, &Handle<Mesh>), Without<Children>>,
	mesh_assets: ResMut<Assets<Mesh>>,
	leaf_mesh_handle: Res<LeafMeshHandle>,
	mut commands: Commands
) {
	for (leaf, handle) in leaves.iter() {
		if *handle == leaf_mesh_handle.0 {
			if let Some(mesh) = mesh_assets.get(handle) {
				if let Some(mut collider) = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::ConvexHull) {
					collider.set_scale(Vec3::splat(1.), 1);
					let physics = commands.spawn(VisibilityBundle::default()).id();

					commands.entity(leaf).insert((
						collider,
						ColliderMassProperties::Density(0.1),
						RigidBody::Fixed,
						ActiveEvents::COLLISION_EVENTS,
						LeafMarker
					)).add_child(physics);
				};
			}
		}
	}
}

fn handle_leaf_collisions(
	mut collision_events: EventReader<CollisionEvent>,
	leaves: Query<Entity, With<LeafMarker>>,
	mut commands: Commands
) {
	for collision_event in collision_events.read() {
		match collision_event {
			CollisionEvent::Started(a, b,_) => {
				if leaves.contains(*a) {make_leaf_falling(a, &mut commands);}
				if leaves.contains(*b) {make_leaf_falling(b, &mut commands);}
			}
			CollisionEvent::Stopped(_, _, _) => {}
		}
	}
}

fn make_leaf_falling(entity: &Entity, commands: &mut Commands) {
	info!("Fall down!");
	commands.entity(*entity).remove::<RigidBody>();
	commands.entity(*entity).insert((
		RigidBody::Dynamic,
		GravityScale(1.0)
	));
}