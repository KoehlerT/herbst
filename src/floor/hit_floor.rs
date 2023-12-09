use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::{shooter::BallMarker, floor::{BallLandEvent, LeafLandEvent}, tree::LeafMarker};

pub fn floor_hits(
    mut collision_events: EventReader<CollisionEvent>,
	floor: Query<Entity, With<super::FloorMarker>>,
	balls: Query<Entity, With<BallMarker>>,
	leaves: Query<Entity, With<LeafMarker>>,
	mut ball_events: EventWriter<super::BallLandEvent>,
	mut leaf_events: EventWriter<super::LeafLandEvent>
) {
	let floor = floor.single();
    for collision_event in collision_events.read() {
		match collision_event {
			CollisionEvent::Started(a, b,_) => {
				if let Some((partner, t)) = get_partner(a, b, &floor, &balls, &leaves) {
					match t {
						FloorHitType::Ball => {ball_events.send(BallLandEvent{ball: partner})},
						FloorHitType::Leaf => {leaf_events.send(LeafLandEvent {leaf: partner})}
					}
				}
			}
			CollisionEvent::Stopped(_,_ ,_ ) => {}
		}
    }
}

enum FloorHitType {
	Ball, Leaf
}
fn get_partner(a: &Entity, b: &Entity, floor: &Entity, balls: &Query<Entity, With<BallMarker>>, leaves: &Query<Entity, With<LeafMarker>>) -> Option<(Entity, FloorHitType)> {
	let partner;
	if a == floor {
		partner = b;
	} else if b == floor {
		partner = a;
	} else {
		return None;
	}
	if balls.contains(*partner) {
		return Some((*partner, FloorHitType::Ball))
	}
	if leaves.contains(*partner) {
		return Some((*partner, FloorHitType::Leaf))
	}
	return None;
}