use bevy::{
    prelude::*,
    sprite::collide_aabb::{self, *},
};
use rand::prelude::*;

use super::paddle::*;
use super::*;

pub const BALL_SIZE: f32 = 30.;
pub const BALL_SIZE_HALF: f32 = BALL_SIZE / 2.;
pub const BALL_SPEED: f32 = 300.;

#[derive(Component)]
struct Ball {
    direction: Vec3,
}

impl Ball {
    fn new() -> Ball {
        Ball {
            direction: Vec3::ZERO,
        }
    }

    fn set_dir_to_random(&mut self) {
        let mut rng = rand::thread_rng();

        self.direction = Vec3::new(
            rng.gen_range(-1000.0_f32..=1000.0_f32),
            rng.gen_range(-1000.0_f32..=1000.0_f32),
            0.,
        )
        .try_normalize()
        .unwrap_or_else(|| Vec3::new(45.0_f32.to_radians().cos(), 45.0_f32.to_radians().sin(), 0.));
    }
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GoalEvent>()
            .add_startup_system(spawn_ball_system)
            .add_systems(
                (
                    move_ball_system,
                    handle_ball_collision_system.after(move_ball_system),
                    handle_ball_score_system,
                )
                    .in_set(OnUpdate(GameState::Ongoing)),
            );
    }
}

/// If left_scored is false, then right player scored.
pub struct GoalEvent {
    pub left_scored: bool,
}

fn spawn_ball_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn ball
    let mut ball = Ball::new();
    ball.set_dir_to_random();

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/ball.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
                ..default()
            },
            ..default()
        },
        ball,
    ));
}

fn move_ball_system(time: Res<Time>, mut ball_query: Query<(&mut Transform, &Ball)>) {
    let (mut ball_transform, ball) = ball_query.single_mut();
    ball_transform.translation += BALL_SPEED * ball.direction * time.delta_seconds();
}

fn handle_ball_collision_system(
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    paddles_query: Query<&Transform, (Or<(With<LeftPaddle>, With<RightPaddle>)>, Without<Ball>)>,
) {
    let (mut ball_transform, mut ball) = ball_query.single_mut();
    let ball_top = ball_transform.translation.y + BALL_SIZE_HALF;
    let ball_bottom = ball_transform.translation.y - BALL_SIZE_HALF;
    let mut ball_angle = if ball.direction.x < 0. {
        if ball.direction.y < 0. {
            -180.0 - ball.direction.y.asin().to_degrees()
        } else {
            ball.direction.x.acos().to_degrees()
        }
    } else {
        ball.direction.y.asin().to_degrees()
    };

    // Check if ball collides on upper wall
    if ball_top > WINDOW_HEIGHT_HALF {
        ball_transform.translation.y = WINDOW_HEIGHT_HALF - BALL_SIZE_HALF;
        ball_angle = -ball_angle;
        ball.direction = Vec3::new(
            ball_angle.to_radians().cos(),
            ball_angle.to_radians().sin(),
            0.,
        );
    }

    // Check if ball collides on lower wall
    if ball_bottom < -WINDOW_HEIGHT_HALF {
        ball_transform.translation.y = -WINDOW_HEIGHT_HALF + BALL_SIZE_HALF;
        ball_angle = -ball_angle;
        ball.direction = Vec3::new(
            ball_angle.to_radians().cos(),
            ball_angle.to_radians().sin(),
            0.,
        );
    }

    // Check ball collision on each paddle
    for paddle_transform in paddles_query.iter() {
        let ball_pos = ball_transform.translation;
        let ball_size = Vec2::new(BALL_SIZE, BALL_SIZE);
        let paddle_pos = paddle_transform.translation;
        let paddle_size = Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT);

        if let Some(collision) = collide_aabb::collide(ball_pos, ball_size, paddle_pos, paddle_size)
        {
            match collision {
                Collision::Top => {
                    ball_transform.translation.y =
                        paddle_pos.y + PADDLE_HEIGHT_HALF + BALL_SIZE_HALF;
                    ball_angle = -ball_angle;
                    ball.direction = Vec3::new(
                        ball_angle.to_radians().cos(),
                        ball_angle.to_radians().sin(),
                        0.,
                    );
                }
                Collision::Bottom => {
                    ball_transform.translation.y =
                        paddle_pos.y - PADDLE_HEIGHT_HALF - BALL_SIZE_HALF;
                    ball_angle = -ball_angle;
                    ball.direction = Vec3::new(
                        ball_angle.to_radians().cos(),
                        ball_angle.to_radians().sin(),
                        0.,
                    );
                }
                Collision::Left => {
                    ball_transform.translation.x =
                        paddle_pos.x - PADDLE_WIDTH_HALF - BALL_SIZE_HALF;
                    ball_angle = 180.0 - ball_angle;
                    ball.direction = Vec3::new(
                        ball_angle.to_radians().cos(),
                        ball_angle.to_radians().sin(),
                        0.,
                    );
                }
                Collision::Right => {
                    ball_transform.translation.x =
                        paddle_pos.x + PADDLE_WIDTH_HALF + BALL_SIZE_HALF;
                    ball_angle = 180.0 - ball_angle;
                    ball.direction = Vec3::new(
                        ball_angle.to_radians().cos(),
                        ball_angle.to_radians().sin(),
                        0.,
                    );
                }
                Collision::Inside => {
                    ball_transform.translation.x = if ball_transform.translation.x < 0. {
                        paddle_pos.x + PADDLE_WIDTH_HALF + BALL_SIZE_HALF
                    } else {
                        paddle_pos.x - PADDLE_WIDTH_HALF - BALL_SIZE_HALF
                    };
                    ball_transform.translation.y = 0.;
                    ball_angle = 180.0;
                    ball.direction = Vec3::new(
                        ball_angle.to_radians().cos(),
                        ball_angle.to_radians().sin(),
                        0.,
                    );
                }
            }
        }
    }
}

fn handle_ball_score_system(
    mut goal_event_writer: EventWriter<GoalEvent>,
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
) {
    let (mut ball_transform, mut ball) = ball_query.single_mut();
    let ball_left = ball_transform.translation.x - BALL_SIZE_HALF;
    let ball_right = ball_transform.translation.x + BALL_SIZE_HALF;

    // Right player scores
    if ball_right < -WINDOW_WIDTH_HALF {
        goal_event_writer.send(GoalEvent { left_scored: false });
        ball_transform.translation = Vec3::new(0., 0., 0.);
        ball.set_dir_to_random();
    }
    // Left player scores
    else if ball_left > WINDOW_WIDTH_HALF {
        goal_event_writer.send(GoalEvent { left_scored: true });
        ball_transform.translation = Vec3::new(0., 0., 0.);
        ball.set_dir_to_random();
    }
}
