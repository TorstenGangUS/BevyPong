mod physics;


use bevy::{math::VectorSpace, prelude::*, window::WindowResolution};

use crate::physics::{Dynamic, components::{collider::Collider, velocity::Velocity}};

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
struct Ball {
    velocity: Vec3//this should be a vec2...
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
        .add_systems(Update, (
            move_paddle,
        ))
        .add_systems(PostUpdate, (
            bounce_off_screen,
            paddle_collisions
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
        Dynamic,
    ));
}


fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    create_paddle(&mut commands, 500.0, Side::Left);
    create_paddle(&mut commands, 500.0, Side::Right);

    commands.spawn((
        Ball{
            velocity: Vec3::new(300.0, 0.0, 0.0)
        },
        Velocity::new(Vec2::new(300.0, 0.0)),
        Sprite::from_color(
            Color::WHITE,
            Vec2::new(10.0, 10.0)
        ),
        Transform::from_xyz(0.0, 0.0 ,0.0),
        Collider::new(Vec2::new(10.0, 10.0)),
        Dynamic,
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


fn bounce_off_screen(
    mut ball_query: Query<(&mut Transform, &mut Ball, &Sprite)>,
    window: Single<&Window, With<bevy::window::PrimaryWindow>>,
) {
    let half_height = window.height() / 2.0;
    

    for (mut transform, mut ball, sprite) in &mut ball_query {
        let ball_radius = sprite.custom_size.unwrap().x/2.0;//the ball has a specified size
        //should I be using sprite for this? or should this be from the ball component?


        //bounce off of the top of the screen
        if transform.translation.y + ball_radius >= half_height {
            transform.translation.y = half_height - ball_radius;
            ball.velocity.y = - ball.velocity.y
        }
        // bottom of the screen
        if transform.translation.y - ball_radius <= -half_height {
            transform.translation.y = -half_height + ball_radius;
            ball.velocity.y = - ball.velocity.y
        }
    }
}

fn paddle_collisions(
    mut ball_query: Query<(&mut Transform, &mut Ball, &Sprite), Without<Paddle>>,
    paddles: Query<(&Transform, &Sprite), (With<Paddle>, Without<Ball>)>
) {
    for (mut ball_transform, mut ball, ball_sprite) in &mut ball_query {
        let ball_pos = ball_transform.translation.xy();
        let ball_size = ball_sprite.custom_size.unwrap();
        let ball_radius = ball_size.x / 2.0;
        for (paddle_transform, paddle_sprite) in &paddles {
            let paddle_size = paddle_sprite.custom_size.unwrap();
            let paddle_pos = paddle_transform.translation.xy();
            let overlap = !(
                ball_pos.x + ball_radius < paddle_pos.x - paddle_size.x / 2.0 ||
                ball_pos.x - ball_radius > paddle_pos.x + paddle_size.x / 2.0 ||
                ball_pos.y + ball_radius < paddle_pos.y - paddle_size.y / 2.0 ||
                ball_pos.y - ball_radius > paddle_pos.y + paddle_size.y / 2.0
            );

            if overlap {
                ball.velocity.x = -ball.velocity.x;
                if ball.velocity.x > 0.0 {
                    ball_transform.translation.x = paddle_pos.x + paddle_size.x / 2.0 + ball_radius;
                } else {
                    ball_transform.translation.x = paddle_pos.x - paddle_size.x / 2.0 - ball_radius;
                }
            }
        }
    }
}