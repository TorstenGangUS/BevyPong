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