use bevy::{pbr::MaterialExtractEntitiesNeedingSpecializationSystems, prelude::*};

use crate::physics::{components::{collider::Collider, velocity::Velocity}, events::collision_event::CollisionMessage};

pub mod components;
pub mod events;
// pub mod events;
// mod systems;

#[derive(Component)]
pub struct Dynamic;

#[derive(Component)]
pub struct Static;


#[derive(Component)]
pub struct Controlled;


//I am doing very simple collisions, because this is pong
// I might revist here and put in spatial hash based collisions
// but, for now I will have at most 30 different things total
// this is fine.


// fn swept_aabb_2d(
//     a_pos: Vec2,
//     a_vel: Vec2,
//     a_half: Vec2,
//     b_pos: Vec2,
//     b_half: Vec2,
// ) -> Option<(f32, Vec2)> {

//     let inv_entry = Vec2::new(
//         if a_vel.x > 0.0 { (b_pos.x - b_half.x) - (a_pos.x + a_half.x) }
//         else { (b_pos.x + b_half.x) - (a_pos.x - a_half.x) },
//         if a_vel.y > 0.0 { (b_pos.y - b_half.y) - (a_pos.y + a_half.y) }
//         else { (b_pos.y + b_half.y) - (a_pos.y - a_half.y) },
//     );

//     let inv_exit = Vec2::new(
//         if a_vel.x > 0.0 { (b_pos.x + b_half.x) - (a_pos.x - a_half.x) }
//         else { (b_pos.x - b_half.x) - (a_pos.x + a_half.x) },
//         if a_vel.y > 0.0 { (b_pos.y + b_half.y) - (a_pos.y - a_half.y) }
//         else { (b_pos.y - b_half.y) - (a_pos.y + a_half.y) },
//     );

//     let entry = Vec2::new(
//         if a_vel.x == 0.0 { f32::NEG_INFINITY } else { inv_entry.x / a_vel.x },
//         if a_vel.y == 0.0 { f32::NEG_INFINITY } else { inv_entry.y / a_vel.y },
//     );

//     let exit = Vec2::new(
//         if a_vel.x == 0.0 { f32::INFINITY } else { inv_exit.x / a_vel.x },
//         if a_vel.y == 0.0 { f32::INFINITY } else { inv_exit.y / a_vel.y },
//     );

//     let entry_time = entry.x.max(entry.y);
//     let exit_time = exit.x.min(exit.y);

//     if entry_time > exit_time || exit_time < 0.0 || entry_time > 1.0 {
//         return None;
//     }
//     if entry_time.is_infinite() {
//         return None
//     }

//     let normal = if entry.x > entry.y {
//         if a_vel.x < 0.0 { Vec2::X } else { Vec2::NEG_X }
//     } else {
//         if a_vel.y < 0.0 { Vec2::Y } else { Vec2::NEG_Y }
//     };

//     Some((entry_time, normal))
// }

//TODO change this to take a collider nad position instead of a_half
// and give collider a function that gives its lower left and upper right corner


fn swept_aabb_2d(
    a_pos: Vec2,
    a_collider: &Collider,
    relative_velocity: Vec2,
    b_pos: Vec2,
    b_collider: &Collider,
) -> Option<(f32, Vec2)> {
    let a_min = a_collider.min(a_pos);
    let a_max = a_collider.max(a_pos);
    let b_min = b_collider.min(b_pos);
    let b_max = b_collider.max(b_pos);

    let mut inv_entry = Vec2::ZERO;
    let mut inv_exit = Vec2::ZERO;

    //normally this broad phase would be done in an outer loop
    // so as to not recompute the broad phase collider every time
    // however, I am making pong. I might make that change later..
    // but there are bigger fish to fry.
    let bpc =  a_collider.broad_phase_check(relative_velocity);
    if !bpc.intersects(a_pos, b_collider, b_pos) {
        return None;
    }

    // find the distance between the objects on the near and far sides for both x and y 
    if relative_velocity.x > 0.0
    { 
        inv_entry.x = b_min.x - a_max.x;  
        inv_exit.x = b_max.x - a_min.x;
    }
    else 
    { 
        inv_entry.x = b_max.x - a_min.x;  
        inv_exit.x = b_min.x - a_max.x;  
    } 

    if relative_velocity.y > 0.0
    { 
        inv_entry.y = b_min.y - a_max.y;  
        inv_exit.y = b_max.y - a_min.y;  
    }
    else 
    { 
        inv_entry.y = b_max.y - a_min.y;  
        inv_exit.y = b_min.y - a_max.y;  
    }

    let mut entry = Vec2::ZERO;
    let mut exit = Vec2::ZERO;

    if relative_velocity.x == 0.0
    { 
        entry.x = f32::NEG_INFINITY; 
        exit.x = f32::INFINITY; 
    } 
    else 
    { 
        entry.x = inv_entry.x / relative_velocity.x;
        exit.x = inv_exit.x / relative_velocity.x;
    } 

    if relative_velocity.y == 0.0 
    { 
        entry.y = f32::NEG_INFINITY; 
        exit.y = f32::INFINITY; 
    } 
    else 
    { 
        entry.y = inv_entry.y / relative_velocity.y; 
        exit.y = inv_exit.y / relative_velocity.y; 
    }

    let entry_time = entry.x.max(entry.y);
    let exit_time = exit.x.min(exit.y);

    //if entry_time > exit_time || (entry.x < 0.0 && entry.y < 0.0) || entry.x > 1.0 || entry.y > 1.0 
    if entry_time > exit_time || exit_time < 0.0 || entry_time > 1.0 
    { 
        // println!("{entry_time} {exit_time}");
        // normalx = 0.0; 
        // normaly = 0.0; 
        None
    } else // if there was a collision 
    { 
        // calculate normal of collided surface
        if entry.x > entry.y 
        { 
            if entry.x < 0.0
            { 
                Some((entry_time, Vec2::new(1.0, 0.0)))
            } 
            else 
            {
                Some((entry_time, Vec2::new(-1.0, 0.0)))
            } 
        } 
        else 
        { 
            if entry.y < 0.0 
            { 
                Some((entry_time, Vec2::new(0.0, 1.0)))
            } 
            else 
            { 
                Some((entry_time, Vec2::new(0.0, -1.0)))
            } 
        } // return the time of collisionreturn entryTime; 
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn moving_right_hits_left_face() {
        let result = swept_aabb_2d(
            Vec2::new(0.0, 0.0),
            &Collider::new(Vec2::new(5.0, 5.0)),
            Vec2::new(10.0, 0.0),
            Vec2::new(5.0, 0.0),
            &Collider::new(Vec2::new(5.0, 5.0)),
        );

        assert!(result.is_some());
    }

    #[test]
    fn moving_right_misses_left_face() {
        let result2 = swept_aabb_2d(
            Vec2::new(0.0, 0.0),
            &Collider::new(Vec2::new(5.0, 5.0)),
            Vec2::new(10.0, 0.0),
            Vec2::new(5.0, 10.0),
            &Collider::new(Vec2::new(5.0, 5.0)),
        );

        assert!(result2.is_none());   
    }
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

        let a_pos = transform_a.translation.xy();
        if let Some((t, normal)) = swept_aabb_2d(
            a_pos,
            collider_a,
            relative_velocity,
            transform_b.translation.xy(),
            collider_b
        ) {
            //TODO this doesn't work for dynamic...
            // should I use relative velocity here? probably? IDK.
            
            let hit_pos = a_pos + (velocity_a.xy() *delta) * t;
            messages.write(CollisionMessage{
                a:entity_a,
                b:entity_b,
                normal,
                time:t,
                impact_point: hit_pos
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
            let a_pos = transform_a.translation.xy();
            if let Some((t, normal)) = swept_aabb_2d(
                a_pos,
                collider_a,
                relative_velocity,
                transform_b.translation.xy(),
                collider_b
            ) {
                
                let hit_pos = a_pos + (velocity_a.xy() *delta) * t;
                println!(
                    "Hit{} - Start:{} {}, rel:{}, b:{} {}",
                    hit_pos,
                    a_pos,
                    collider_a.half_bounds(),
                    relative_velocity,
                    transform_b.translation.xy(),
                    collider_b.half_bounds()
                );
                messages.write(CollisionMessage{
                    a:entity_a,
                    b:entity_b,
                    normal,
                    time:t,
                    impact_point: hit_pos
                });
            }
        }
    }
}

#[derive(Component)]
struct Moved;

// fn handle_collisions(
//     mut messages: MessageReader<CollisionMessage>,
//     mut query: Query<(Option<&mut Velocity>, &mut Transform, &Collider)>,
// ) {
//     for message in messages.read() {
        
//         let Ok((mut vel_a, mut transform, collider)) = query.get_mut(message.a) else {
//             continue;
//         };
//         let Some(mut vel_a) = vel_a else { continue };

//         //position the block...
//         // float normalx, normaly; 
//         // float collisiontime = SweptAABB(box, block, out normalx, out normaly); 
//         // box.x += box.vx * collisiontime; 
//         // box.y += box.vy * collisiontime; 
//         // float remainingtime = 1.0f - collisiontime;

//         let normal = message.normal;
//         let v = vel_a.xy();
//         let reflected = v - 2.0 * v.dot(normal) * normal;
//         let new_offset = message.impact_point;
//         transform.translation.x = new_offset.x;
//         transform.translation.y = new_offset.y;
//         vel_a.set_xy(reflected);
//         //reflect!
//         // deflectbox.vx *= remainingtime; 
//         // box.vy *= remainingtime; 
//         // if (abs(normalx) > 0.0001f) box.vx = -box.vx; 
//         // if (abs(normaly) > 0.0001f) box.vy = -box.vy;
//         //AND prevent from moving more this frame...
        
//     }
// }

fn handle_collisions(
    mut commands: Commands,
    mut messages: MessageReader<CollisionMessage>,
    mut query: Query<(Entity,
        &mut Velocity,
        &mut Transform,
    )>,
    time: Res<Time<Fixed>>,
) {
    let delta = time.delta_secs();

    for message in messages.read() {

        let Ok((entity, mut vel_a, mut transform)) =
            query.get_mut(message.a)
        else {
            continue;
        };

        //
        // Move to exact impact position
        //
        transform.translation.x = message.impact_point.x;
        transform.translation.y = message.impact_point.y;

        //
        // Reflect velocity
        //
        let normal = message.normal;
        let velocity = vel_a.xy();

        let reflected =
            velocity - 2.0 * velocity.dot(normal) * normal;

        vel_a.set_xy(reflected);

        //
        // Move remaining frame time using reflected velocity
        //
        let remaining_time = 1.0 - message.time;

        transform.translation +=
            (reflected * delta * remaining_time)
                .extend(0.0);
        commands.entity(entity).insert(Moved);
    }
}



fn move_object(
    mut query: Query<(&mut Transform, &Velocity), Without<Moved>>,
    time: Res<Time<Fixed>>,
) {
    let delta = time.delta_secs();
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.xy().extend(0.0) * delta; 
    }
}

fn reset_move(mut commands: Commands,
    query: Query<Entity, With<Moved>>,
) {
    for entity in query {
        //TODO batch this
        commands.entity(entity).remove::<Moved>();
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
                ).before(
                    handle_collisions
                ),
                handle_collisions,
                move_object.after(handle_collisions),
                reset_move.after(move_object)
            ))
        ;
    }
}