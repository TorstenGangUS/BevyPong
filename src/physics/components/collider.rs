use bevy::prelude::*;


#[derive(Component)]
pub struct Collider(Vec2);


impl Collider {
    pub fn new(bounds: Vec2) -> Self {
        Self(bounds)
    }

    pub fn bounds(&self) -> Vec2 {
        self.0
    }

    pub fn half_bounds(&self) -> Vec2 {
        self.0/2.0
    }
}