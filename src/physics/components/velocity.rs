use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(Vec2);


impl Velocity {
    pub fn new(vel: Vec2) -> Self {
        Self(vel)
    }

    pub fn xy(&self) -> Vec2 {
        self.0
    }
}