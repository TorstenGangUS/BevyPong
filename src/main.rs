mod physics;


use bevy::{math::VectorSpace, prelude::*, window::WindowResolution};

use crate::physics::{Dynamic, Static, components::{collider::Collider, velocity::Velocity}};

//TODO how to organize this...
#[derive(Component)]
struct Paddle {
    speed: f32,
    side: Side,
}

#[derive(Component)]
enum Side {
    Left,
    Right
}

#[derive(Component)]
struct Ball;


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
        .add_systems(Update, (
            move_paddle,
        ))

        .run()
    ;
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
    ));
}


fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    create_paddle(&mut commands, 500.0, Side::Left);
    create_paddle(&mut commands, 500.0, Side::Right);

    commands.spawn((
        Ball,
        Velocity::new(Vec2::new(300.0, 150.0)),
        Sprite::from_color(
            Color::WHITE,
            Vec2::new(10.0, 10.0)
        ),
        Transform::from_xyz(0.0, 0.0 ,0.0),
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
