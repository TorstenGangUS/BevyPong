use bevy::prelude::*;


#[derive(Component)]
pub struct Collider{
    x: f32,
    y: f32,
    h: f32,
    w: f32,
}

//this is an AABB centered around an entities transform

impl Collider {
    const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        h: 0.0,
        w: 0.0,
    };

    pub fn new(bounds: Vec2) -> Self {
        assert!(bounds.x > 0.0);
        assert!(bounds.y > 0.0);

        let x = -bounds.x/2.0;
        let y = -bounds.y/2.0;
        Self{
            x,
            y,
            w:bounds.x,
            h:bounds.y,
        }
    }

    pub fn bounds(&self) -> Vec2 {
        Vec2::new(self.w, self.h)
    }

    pub fn half_bounds(&self) -> Vec2 {
        Vec2::new(self.w, self.h)/2.0
    }

    pub fn min(&self, pos:Vec2) -> Vec2 {
        pos + Vec2::new(self.x, self.y)
    }

    pub fn max(&self, pos:Vec2) -> Vec2 {
        pos + Vec2::new(self.x, self.y) + Vec2::new(self.w, self.h)
    }

    pub fn broad_phase_check(
        &self,
        rel_velocity: Vec2,
    ) -> Collider { 
        let mut broadphasebox = Collider::ZERO;
        broadphasebox.x = if rel_velocity.x > 0.0 { self.x } else { self.x + rel_velocity.x };
        broadphasebox.y = if rel_velocity.y > 0.0 { self.y } else { self.y + rel_velocity.y };
        broadphasebox.w = if rel_velocity.x > 0.0 { rel_velocity.x + self.w } else { self.w - rel_velocity.x };
        broadphasebox.h = if rel_velocity.y > 0.0 { rel_velocity.y + self.h } else { self.h - rel_velocity.y };

        return broadphasebox; 
    }

    pub fn intersects(&self, my_pos: Vec2, other: &Self, other_pos: Vec2) -> bool
    { 
        !(
            my_pos.x + self.x + self.w < other.x + other_pos.x ||
            my_pos.x + self.x > other.x + other_pos.x + other.w||
            my_pos.y + self.y + self.h < other.y + other_pos.y ||
            my_pos.y + self.y > other.y + other_pos.y + other.h
        )
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;
    
    #[test]
    fn intersection() {
        let a = Collider::new(Vec2::new(10.0, 10.0));
        let b = Collider::new(Vec2::new(10.0, 10.0));

        assert!(a.intersects(Vec2::new(0.0, 0.0), &b, Vec2::new(2.0, 0.0)));
    }
}