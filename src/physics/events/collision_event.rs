use bevy::prelude::*;


//this should be only visible in physics
#[derive(Message, Debug)]
pub struct CollisionMessage {
    pub a: Entity,
    pub b: Entity,
    pub normal: Vec2,
    pub time: f32, //should this be here?
    pub impact_point: Vec2,
}

#[derive(EntityEvent, Debug)]
pub struct CollisionEvent {
    pub entity: Entity,
    pub target: Entity,
    pub impact_point: Vec2,
    pub my_velocity: Vec2,
    pub target_velocity: Vec2
}