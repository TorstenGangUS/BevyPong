use bevy::prelude::*;

use crate::physics::{components::{collider::Collider, velocity::Velocity}, events::collision_event::CollisionMessage};

pub mod components;
pub mod events;
// pub mod events;
// mod systems;

#[derive(Component)]
pub struct Dynamic;

#[derive(Component)]
pub struct Static;




fn move_object(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time<Fixed>>,
) {
    let delta = time.delta_secs();
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.xy().extend(0.0) * delta; 
    }
}


//I am doing very simple collisions, because this is pong
// I might revist here and put in spatial hash based collisions
// but, for now I will have at most 30 different things total
// this is fine.

// fn swept_aabb_2d(
//     a_pos: Vec2,
//     a_velocity: Vec2,
//     a_half_collider: Vec2,
//     b_pos: Vec2,
//     b_half_collider: Vec2,
// ) -> Option<(f32, Vec2)> {
//     let min_a = a_pos - a_half_collider;
//     let max_a = a_pos + a_half_collider;

//     let min_b = b_pos - b_half_collider;
//     let max_b = b_pos + b_half_collider;

//     let inv = Vec2::new(
//         if a_velocity.x != 0.0 { 1.0 / a_velocity.x } else { f32::INFINITY },
//         if a_velocity.y != 0.0 { 1.0 / a_velocity.y } else { f32::INFINITY },
//     );

//     let mut t1 = Vec2::ZERO;
//     let mut t2 = Vec2::ZERO;

//     // X axis
//     if a_velocity.x > 0.0 {
//         t1.x = (min_b.x - max_a.x) * inv.x;
//         t2.x = (max_b.x - min_a.x) * inv.x;
//     } else {
//         t1.x = (max_b.x - min_a.x) * inv.x;
//         t2.x = (min_b.x - max_a.x) * inv.x;
//     }

//     // Y axis
//     if a_velocity.y > 0.0 {
//         t1.y = (min_b.y - max_a.y) * inv.y;
//         t2.y = (max_b.y - min_a.y) * inv.y;
//     } else {
//         t1.y = (max_b.y - min_a.y) * inv.y;
//         t2.y = (min_b.y - max_a.y) * inv.y;
//     }

//     let entry = t1.x.max(t1.y);
//     let exit = t2.x.min(t2.y);

//     if entry > exit || exit < 0.0 || entry > 1.0 {
//         return None;
//     }

//     // collision normal
//     let normal = if t1.x > t1.y {
//         if a_velocity.x < 0.0 { Vec2::X } else { Vec2::NEG_X }
//     } else {
//         if a_velocity.y < 0.0 { Vec2::Y } else { Vec2::NEG_Y }
//     };

//     Some((entry, normal))
// }

fn swept_aabb_2d(
    a_pos: Vec2,
    a_vel: Vec2,
    a_half: Vec2,
    b_pos: Vec2,
    b_half: Vec2,
) -> Option<(f32, Vec2)> {

    let inv_entry = Vec2::new(
        if a_vel.x > 0.0 { (b_pos.x - b_half.x) - (a_pos.x + a_half.x) }
        else { (b_pos.x + b_half.x) - (a_pos.x - a_half.x) },
        if a_vel.y > 0.0 { (b_pos.y - b_half.y) - (a_pos.y + a_half.y) }
        else { (b_pos.y + b_half.y) - (a_pos.y - a_half.y) },
    );

    let inv_exit = Vec2::new(
        if a_vel.x > 0.0 { (b_pos.x + b_half.x) - (a_pos.x - a_half.x) }
        else { (b_pos.x - b_half.x) - (a_pos.x + a_half.x) },
        if a_vel.y > 0.0 { (b_pos.y + b_half.y) - (a_pos.y - a_half.y) }
        else { (b_pos.y - b_half.y) - (a_pos.y + a_half.y) },
    );

    let entry = Vec2::new(
        if a_vel.x == 0.0 { f32::NEG_INFINITY } else { inv_entry.x / a_vel.x },
        if a_vel.y == 0.0 { f32::NEG_INFINITY } else { inv_entry.y / a_vel.y },
    );

    let exit = Vec2::new(
        if a_vel.x == 0.0 { f32::INFINITY } else { inv_exit.x / a_vel.x },
        if a_vel.y == 0.0 { f32::INFINITY } else { inv_exit.y / a_vel.y },
    );

    let entry_time = entry.x.max(entry.y);
    let exit_time = exit.x.min(exit.y);

    if entry_time > exit_time || exit_time < 0.0 || entry_time > 1.0 {
        return None;
    }

    let normal = if entry.x > entry.y {
        if a_vel.x < 0.0 { Vec2::X } else { Vec2::NEG_X }
    } else {
        if a_vel.y < 0.0 { Vec2::Y } else { Vec2::NEG_Y }
    };

    Some((entry_time, normal))
}


fn detect_dynamic_collisions(
    mut messages: MessageWriter<CollisionMessage>,
    dynamic: Query<(Entity, &Transform, &Velocity, &Collider), With<Dynamic>>,
    f_time: Res<Time<Fixed>>,
) {
    let delta = f_time.delta_secs();
    for [(entity_a, transform_a, velocity_a, collider_a),
         (entity_b, transform_b, velocity_b, collider_b)
        ] in dynamic.iter_combinations() {
        let relative_velocity = (velocity_a.xy() - velocity_b.xy()) * delta;
        println!("Comparing {entity_a} to {entity_b} vel {relative_velocity}");
        if let Some((t, normal)) = swept_aabb_2d(
            transform_a.translation.xy(),
            relative_velocity,
            collider_a.half_bounds(),
            transform_b.translation.xy(),
            collider_b.half_bounds()
        ) {
            println!("collision detected {entity_a} to {entity_b} vel {relative_velocity}");
            messages.write(CollisionMessage{
                a:entity_a,
                b:entity_b,
                normal,
                time:t
            });
        }
    }
}

fn detect_dynamic_to_static_collisions(
    mut messages: MessageWriter<CollisionMessage>,
    dynamic: Query<(Entity, &Transform, &Velocity, &Collider), With<Dynamic>>,
    static_colliders: Query<(Entity, &Transform, &Collider), With<Static>>,
    f_time: Res<Time<Fixed>>,
) {
    let delta = f_time.delta_secs();
    for (entity_a, transform_a, velocity_a, collider_a)
        in &dynamic {
        for (entity_b, transform_b,  collider_b) in &static_colliders {
            let relative_velocity = velocity_a.xy() * delta;
            
            if let Some((t, normal)) = swept_aabb_2d(
                transform_a.translation.xy(),
                relative_velocity,
                collider_a.half_bounds(),
                transform_b.translation.xy(),
                collider_b.half_bounds()
            ) {
                messages.write(CollisionMessage{
                    a:entity_a,
                    b:entity_b,
                    normal,
                    time:t
                });
            }
        }
    }
}

fn handle_collisions(
    mut messages: MessageReader<CollisionMessage>
) {
    for message in messages.read() {
        debug!("{message:?}");
    }
}


pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<CollisionMessage>()
            .add_systems(FixedUpdate, (
                (
                    detect_dynamic_to_static_collisions,
                    detect_dynamic_collisions,
                ).after(
                    handle_collisions    
                ),
                handle_collisions,
                move_object.after(handle_collisions)
            ))
        ;
    }
}