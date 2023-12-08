use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::{shooter::BallMarker, floor::BallLandEvent};

pub fn floor_hits(
    mut collision_events: EventReader<CollisionEvent>,
	floor: Query<Entity, With<super::FloorMarker>>,
	balls: Query<Entity, With<BallMarker>>,
	mut ball_events: EventWriter<super::BallLandEvent>
) {
	let floor = floor.single();
    for collision_event in collision_events.read() {
        println!("Received collision event: {:?}", collision_event);
		match collision_event {
			CollisionEvent::Started(a, b,_) => {
				if let Some((partner, t)) = get_partner(a, b, &floor, &balls) {
					match t {
						FloorHitType::Ball => {ball_events.send(BallLandEvent{ball: partner})},
						FloorHitType::Leaf => {}
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
fn get_partner(a: &Entity, b: &Entity, floor: &Entity, balls: &Query<Entity, With<BallMarker>>) -> Option<(Entity, FloorHitType)> {
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
	return None;
}