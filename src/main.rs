mod physics;


use bevy::{math::VectorSpace, prelude::*, window::WindowResolution};

use crate::physics::{Dynamic, Static, components::{collider::Collider, velocity::Velocity}, events::collision_event::CollisionEvent};

//TODO how to organize this...
#[derive(Component)]
struct Paddle {
    speed: f32,
    side: Side,
}

#[derive(Component, Copy, Clone)]
enum Side {
    Left,
    Right
}

#[derive(Component)]
struct Ball;


#[derive(Component)]
struct Constraint {
    lower_left: Vec2,
    upper_right: Vec2
}

impl Constraint {
    pub fn new(lower_left: Vec2, upper_right: Vec2) -> Self {
        assert!(lower_left.x <= upper_right.x);
        assert!(lower_left.y <= upper_right.y);
        Self { lower_left, upper_right }
    }
}

fn constraint_system(
    mut query: Query<(&mut Transform, &Collider, &Constraint)>,
) {
    for (mut transform, collider, constraint) in &mut query {

        let pos = transform.translation.xy();
        let half = collider.half_bounds();

        let min = constraint.lower_left + half;
        let max = constraint.upper_right - half;

        let clamped = Vec2::new(
            pos.x.clamp(min.x, max.x),
            pos.y.clamp(min.y, max.y),
        );

        transform.translation.x = clamped.x;
        transform.translation.y = clamped.y;
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window:Some(Window{
                resolution: WindowResolution::new(800, 600),
                title: "Pong".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            physics::PhysicsPlugin,
        ))
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (
            move_paddle,
        ))
        .add_systems(
            FixedPostUpdate, 
            constraint_system
        )

        .run()
    ;
}

fn paddle_collision_observer(
    event: On<CollisionEvent>,
    mut vel_query: Query<&mut Velocity>,
    colliders: Query<&Collider>,
    transforms: Query<&Transform>,
) {
    let Ok(paddle_tf) = transforms.get(event.entity) else {
        return;
    };

    let Ok(mut ball_vel) = vel_query.get_mut(event.target) else {
        return;
    };

    let Ok(paddle_collider) = colliders.get(event.entity) else {return};

    let offset = event.impact_point.y - paddle_tf.translation.y;
    let normalized = (offset / paddle_collider.half_bounds().y).clamp(-1.0, 1.0);

    let mut v = ball_vel.xy();

    let speed = v.length();//TODO worry about ball speed exploding or shrinking after collisions...
    let dir_x = v.x.signum();

    let max_angle = 0.85;
    let center_bias = 0.25;

    v.x = dir_x * speed * (1.0 - normalized.abs() * center_bias);
    v.y = normalized * speed * max_angle;

    ball_vel.set_xy(v);
}

fn create_paddle(commands: &mut Commands, speed: f32, side:Side) {
    let start_position =  match side {
        Side::Left => Vec3::new(-350.0, 0.0 , 0.0),
        Side::Right => Vec3::new(350.0, 0.0, 0.0),
    };

    commands.spawn((
        Paddle{
            speed: speed,
            side:side
        },
        Sprite::from_color(
            Color::WHITE,
            Vec2::new(10.0, 100.0)
        ),
        Transform::from_translation(start_position),
        Velocity::new(Vec2::ZERO),
        Collider::new(Vec2::new(10.0, 100.0)),
        Static,
        match side {
            Side::Left => Constraint::new(
                Vec2::new(-360.0, -243.0),
                Vec2::new(-340.0, 243.0),
            ),
            Side::Right => Constraint::new(
                Vec2::new(340.0, -243.0),
                Vec2::new(380.0, 243.0),
            ),
        }
    ))
    .observe(paddle_collision_observer)
    ;
}




fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    create_paddle(&mut commands, 500.0, Side::Left);
    create_paddle(&mut commands, 500.0, Side::Right);

    // commands.spawn((
    //     Ball,
    //     Velocity::new(Vec2::new(-300.0, -150.0)),
    //     Sprite::from_color(
    //         Color::WHITE,
    //         Vec2::new(10.0, 10.0)
    //     ),
    //     Transform::from_xyz(0.0, 70.0 ,0.0),
    //     Collider::new(Vec2::new(10.0, 10.0)),
    //     Dynamic,
    // ));


    // commands.spawn((
    //     Ball,
    //     Velocity::new(Vec2::new(-300.0, -150.0)),
    //     Sprite::from_color(
    //         Color::WHITE,
    //         Vec2::new(10.0, 10.0)
    //     ),
    //     Transform::from_xyz(0.0, -70.0 ,0.0),
    //     Collider::new(Vec2::new(10.0, 10.0)),
    //     Dynamic,
    // ));

    // for i in 0..20 {
    //     commands.spawn((
    //         Ball,
    //         Velocity::new(Vec2::new(0.0, -150.0)),
    //         Sprite::from_color(
    //             Color::WHITE,
    //             Vec2::new(10.0, 10.0)
    //         ),
    //         Transform::from_xyz(0.0 + 15.0 * i as f32, -70.0 + 13.0 * i as f32 ,0.0),
    //         Collider::new(Vec2::new(10.0, 10.0)),
    //         Dynamic,
    //     ));
    // }

    commands.spawn((
        Ball,
        Velocity::new(Vec2::new(-300.0, 0.0)),
        Sprite::from_color(
            Color::WHITE,
            Vec2::new(10.0, 10.0)
        ),
        Transform::from_xyz(200.0, 0.0 ,0.0),
        Collider::new(Vec2::new(10.0, 10.0)),
        Dynamic,
    ));

    commands.spawn((
        Ball,
        Velocity::new(Vec2::new(300.0, 0.0)),
        Sprite::from_color(
            Color::WHITE,
            Vec2::new(10.0, 10.0)
        ),
        Transform::from_xyz(-200.0, 0.0 ,0.0),
        Collider::new(Vec2::new(10.0, 10.0)),
        Dynamic,
    ));

    



    //spawn walls
    commands.spawn((
        Sprite::from_color(
            Color::WHITE,
            Vec2::new(800.0, 10.0)
        ),
        Transform::from_xyz(0.0, 250.0 ,0.0),
        Collider::new(Vec2::new(800.0, 10.0)),
        Static,
    ));
    commands.spawn((
        Sprite::from_color(
            Color::WHITE,
            Vec2::new(800.0, 10.0)
        ),
        Transform::from_xyz(0.0, -250.0 ,0.0),
        Collider::new(Vec2::new(800.0, 10.0)),
        Static,
    ));
}


fn move_paddle(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut paddle_query: Query<(&mut Transform, &Paddle)>,
    time: Res<Time>
) {
    for(mut transform, paddle) in &mut paddle_query {
        let mut direction = 0.0;
        match paddle.side {
            Side::Left => {
                if keyboard.pressed(KeyCode::KeyW) {
                    direction += 1.0;
                }
                if keyboard.pressed(KeyCode::KeyS) {
                    direction -= 1.0;
                } 
            },
            Side::Right => {
                if keyboard.pressed(KeyCode::ArrowUp) {
                    direction += 1.0;
                }
                if keyboard.pressed(KeyCode::ArrowDown) {
                    direction -= 1.0;
                } 
            }
        }
        transform.translation.y += direction * paddle.speed * time.delta_secs();
    }
}
