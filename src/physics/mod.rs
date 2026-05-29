use bevy::{pbr::MaterialExtractEntitiesNeedingSpecializationSystems, prelude::*};

use crate::physics::{components::{collider::Collider, velocity::Velocity}, events::collision_event::{CollisionEvent, CollisionMessage}};

pub mod components;
pub mod events;
// pub mod events;
// mod systems;

#[derive(Component)]
pub struct Dynamic;

#[derive(Component)]
pub struct Static;


//I am doing very simple collisions, because this is pong
// I might revist here and put in spatial hash based collisions
// but, for now I will have at most 30 different things total
// this is fine.


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

    if relative_velocity.y > 0.0 {
        inv_entry.y = b_min.y - a_max.y;  
        inv_exit.y = b_max.y - a_min.y;  
    }
    else {
        inv_entry.y = b_max.y - a_min.y;  
        inv_exit.y = b_min.y - a_max.y;  
    }

    let mut entry = Vec2::ZERO;
    let mut exit = Vec2::ZERO;

    if relative_velocity.x == 0.0 { 
        entry.x = f32::NEG_INFINITY; 
        exit.x = f32::INFINITY; 
    } else { 
        entry.x = inv_entry.x / relative_velocity.x;
        exit.x = inv_exit.x / relative_velocity.x;
    } 

    if relative_velocity.y == 0.0 { 
        entry.y = f32::NEG_INFINITY; 
        exit.y = f32::INFINITY; 
    } else { 
        entry.y = inv_entry.y / relative_velocity.y; 
        exit.y = inv_exit.y / relative_velocity.y; 
    }

    let entry_time = entry.x.max(entry.y);
    let exit_time = exit.x.min(exit.y);

    if entry_time > exit_time || exit_time < 0.0 || entry_time > 1.0 
    { 
        None
    } else // if there was a collision 
    { 
        // calculate normal of collided surface
        if entry.x > entry.y 
        { 
            if relative_velocity.x < 0.0
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
            if relative_velocity.y < 0.0 
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



fn physics_step(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Velocity, &Collider), (With<Dynamic>, Without<Static>)>,
    static_colliders: Query<(Entity, &Transform, &Collider), (With<Static>, Without<Dynamic>)>,
    
    time: Res<Time<Fixed>>,
) {
    let dt = time.delta_secs();
    let snapshot: Vec<(Entity, Vec2, Vec2, Collider)> = query
        .iter()
        .map(|(e, t, v, c)| (
            e,
            t.translation.xy(),
            v.xy(),
            *c,
        ))
        .collect();


    for (entity, mut transform, mut velocity, collider) in &mut query {

        let mut remaining_time = 1.0;
        let mut iterations = 0;

        while remaining_time > 0.0 && iterations < 4 {

            let mut earliest_hit: Option<(f32, Vec2, Vec2)> = None;
            let mut hit_entity: Option<Entity> = None;

            let start_pos = transform.translation.xy();
            let vel = velocity.xy() * dt * remaining_time;
            let mut other_vel_at_impact = Vec2::ZERO;
            // look for static collison
            for (other_entity, other_transform, other_collider) in &static_colliders {

                if let Some((t, normal)) = swept_aabb_2d(
                    start_pos,
                    collider,
                    vel,
                    other_transform.translation.xy(),
                    other_collider,
                ) {
                    if earliest_hit.map_or(true, |(best_t, _, _)| t < best_t) {
                        earliest_hit = Some((t, normal, other_transform.translation.xy()));
                        hit_entity = Some(other_entity);
                        other_vel_at_impact = Vec2::ZERO;
                    }
                }
            }

            // look for dynamic collisions...
            // There is a bug in how this work.. as 
            // we should freeze the world figure out collisions
            // take a step and continue.. but, I don't care about that level of accuracy
            // I am networking this.. so this will kick me later I suppose.
            for (other_entity, other_pos, other_vel, other_collider) in &snapshot {

                if *other_entity == entity {
                    continue;
                }

                let relative_velocity =
                    (velocity.xy() - *other_vel) * dt * remaining_time;

                if let Some((t, normal)) = swept_aabb_2d(
                    start_pos,
                    collider,
                    relative_velocity,
                    *other_pos,
                    other_collider,
                ) {
                    if earliest_hit.map_or(true, |(best_t, _, _)| t < best_t) {
                        earliest_hit = Some((t, normal, *other_pos));
                        hit_entity = Some(*other_entity);
                        other_vel_at_impact = *other_vel;
                    }
                }
            }

            // no collision, just move!
            let Some((t, normal, _hit_pos)) = earliest_hit else {
                transform.translation += vel.extend(0.0);
                break;
            };

            transform.translation += (vel * t).extend(0.0);


            //this isn't quite right as this is the center of the object
            // at time of impact. not the actual point where the
            // impact occurs.
            let impact_point = transform.translation.xy();
            
            // tiny push to prevent re-collision jitter
            transform.translation += (normal * 0.001).extend(0.0);


            let velocity_at_impact = velocity.xy();
            let target = hit_entity.unwrap();
            //TRIGGER collision events
            commands.entity(entity).trigger( |entity|
                CollisionEvent{
                    entity:entity,
                    target:target,
                    impact_point: impact_point,
                    my_velocity: velocity_at_impact,
                    target_velocity: other_vel_at_impact
                }
            );

            commands.entity(target).trigger( |target|
                CollisionEvent{
                    entity:target,
                    target:entity,
                    impact_point: impact_point,
                    my_velocity: other_vel_at_impact,
                    target_velocity: velocity_at_impact
                }
            );
            

            //handle reflection

            let v = velocity.xy();
            let reflected =
                v - 2.0 * v.dot(normal) * normal;

            velocity.set_xy(reflected);

            remaining_time *= 1.0 - t;

            iterations += 1;
        }
    }
}


pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<CollisionMessage>()
            // .add_systems(FixedUpdate, (
            //     (
            //         detect_dynamic_to_static_collisions,
            //         detect_dynamic_collisions,
            //     ).before(
            //         handle_collisions
            //     ),
            //     handle_collisions,
            //     move_object.after(handle_collisions),
            //     reset_move.after(move_object)
            // ))
            .add_systems(FixedUpdate, physics_step)
        ;
    }
}