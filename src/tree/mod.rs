use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

pub struct TreePlugin;

impl Plugin for TreePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup_tree)
			.add_systems(Startup, spawn_flying_leaves)
			.add_systems(Update, (spawn_leaves, update_flying_leaves, add_leaf_colliders, handle_leaf_collisions));
	}
}

const FLYING_LEAVES_COUNT: u32 = 350;
const LEAF_PROBABILITY : f32 = 0.8;

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
pub struct LeafMarker;

fn spawn_leaves(
	mut meshes: Query<(Entity, &Handle<Mesh>, &mut Tree), With<Tree>>,
	mesh_assets: ResMut<Assets<Mesh>>,
	ass: Res<AssetServer>,
    mut commands: Commands,
) {

	for (tree_entity, mesh_handle, mut tree) in meshes.iter_mut() {
		if tree.has_leaves {
			continue;
		}
		
		if let Some(mesh) = mesh_assets.get(mesh_handle) {

			let leaf = ass.load("assets.glb#Scene2");

			if let Some(vertices) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
				#[allow(for_loops_over_fallibles)]
				for v in vertices.as_float3() {
					for vv in v {

						if rand::thread_rng().gen::<f32>() < LEAF_PROBABILITY {continue;}
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
			// add tree collider
			if let Some(collider) = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh) {
				commands.entity(tree_entity).insert((
					collider,
					RigidBody::Fixed
				));
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

#[derive(Component)]
struct HandledAddColider;

fn add_leaf_colliders(
	leaves: Query<(Entity, &Handle<Mesh>), Without<HandledAddColider>>,
	leaf_mesh_handle: Res<LeafMeshHandle>,
	mut commands: Commands
) {
	for (leaf, handle) in leaves.iter() {
		if *handle == leaf_mesh_handle.0 {
			commands.entity(leaf).insert((
				Collider::cuboid(0.05, 0.01, 0.08),
				ColliderMassProperties::Density(0.1),
				RigidBody::Fixed,
				LeafMarker
			));
		}
		commands.entity(leaf).insert(HandledAddColider);
	}
}

fn handle_leaf_collisions(
	mut collision_events: EventReader<CollisionEvent>,
	leaves: Query<Entity, (With<LeafMarker>, Without<ActiveEvents>)>,
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
	commands.entity(*entity).insert((
		RigidBody::Dynamic,
		GravityScale(1.0),
		ActiveEvents::COLLISION_EVENTS,
	));
}