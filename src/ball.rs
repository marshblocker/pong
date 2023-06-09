// Copyright © 2023 Gabriel Kenneth L. Marinas <gabrielkennethmarinas@gmail.com>
// This work is free. You can redistribute it and/or modify it under the
// terms of the Do What The Fuck You Want To Public License, Version 2,
// as published by Sam Hocevar. See the COPYING file for more details.

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
pub const BALL_FREEZE_DURATION_SECONDS: f32 = 2.0;

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

        let random_angle = if random::<f32>() < 0.5 {
            // Will point to the right paddle
            rng.gen_range(-45.0..=45.0_f32)
        } else {
            // Will point to the left paddle
            rng.gen_range(135.0..=225.0_f32)
        };

        self.direction = Vec3::new(
            random_angle.to_radians().cos(),
            random_angle.to_radians().sin(),
            0.,
        );
    }
}

/// This timer is used to temporarily freeze the ball at every start of a round.
#[derive(Resource)]
struct FreezeBallTimer(Timer);

impl Default for FreezeBallTimer {
    fn default() -> Self {
        FreezeBallTimer(Timer::from_seconds(
            BALL_FREEZE_DURATION_SECONDS,
            TimerMode::Once,
        ))
    }
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FreezeBallTimer>()
            .add_event::<GoalEvent>()
            .add_startup_system(spawn_ball_system)
            .add_system(reset_freeze_ball_timer.in_schedule(OnEnter(GameState::Ongoing)))
            .add_systems(
                (
                    move_ball_system,
                    handle_ball_collision_system.after(move_ball_system),
                    handle_ball_score_system,
                    tick_freeze_ball_timer_system,
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

fn move_ball_system(
    mut ball_query: Query<(&mut Transform, &Ball)>,
    time: Res<Time>,
    freeze_ball_timer: Res<FreezeBallTimer>,
) {
    if freeze_ball_timer.0.paused() {
        let (mut ball_transform, ball) = ball_query.single_mut();
        ball_transform.translation += BALL_SPEED * ball.direction * time.delta_seconds();
    }
}

fn handle_ball_collision_system(
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    paddles_query: Query<&Transform, (With<Paddle>, Without<Ball>)>,
) {
    let (mut ball_transform, mut ball) = ball_query.single_mut();
    let ball_top = ball_transform.translation.y + BALL_SIZE_HALF;
    let ball_bottom = ball_transform.translation.y - BALL_SIZE_HALF;

    if ball_top > WINDOW_HEIGHT_HALF || ball_bottom < -WINDOW_HEIGHT_HALF {
        ball.direction[1] *= -1.;
    }

    for paddle_transform in paddles_query.iter() {
        let ball_pos = ball_transform.translation;
        let ball_size = Vec2::new(BALL_SIZE, BALL_SIZE);
        let paddle_pos = paddle_transform.translation;
        let paddle_size = Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT);

        if let Some(collision) = collide_aabb::collide(ball_pos, ball_size, paddle_pos, paddle_size)
        {
            match collision {
                Collision::Top | Collision::Bottom => ball.direction[1] *= -1.,
                Collision::Left | Collision::Right => ball.direction[0] *= -1.,
                Collision::Inside => {
                    ball_transform.translation.x = if ball_transform.translation.x < 0. {
                        paddle_pos.x + PADDLE_WIDTH_HALF + BALL_SIZE_HALF
                    } else {
                        paddle_pos.x - PADDLE_WIDTH_HALF - BALL_SIZE_HALF
                    };
                    ball.direction = if ball_transform.translation.x < 0. {
                        Vec3::new(1., 0., 0.)
                    } else {
                        Vec3::new(-1., 0., 0.)
                    };
                }
            }
        }
    }
}

fn handle_ball_score_system(
    mut goal_event_writer: EventWriter<GoalEvent>,
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    mut freeze_ball_timer: ResMut<FreezeBallTimer>,
) {
    let (mut ball_transform, mut ball) = ball_query.single_mut();
    let ball_left = ball_transform.translation.x - BALL_SIZE_HALF;
    let ball_right = ball_transform.translation.x + BALL_SIZE_HALF;

    // Left or right player scored
    if ball_right < -WINDOW_WIDTH_HALF || ball_left > WINDOW_WIDTH_HALF {
        let left_scored = ball_left > WINDOW_WIDTH_HALF;

        goal_event_writer.send(GoalEvent { left_scored });
        ball_transform.translation = Vec3::new(0., 0., 0.);
        ball.set_dir_to_random();
        freeze_ball_timer.0.unpause();
    }
}

fn reset_freeze_ball_timer(mut freeze_ball_timer: ResMut<FreezeBallTimer>) {
    freeze_ball_timer.0.reset();
}

fn tick_freeze_ball_timer_system(time: Res<Time>, mut freeze_ball_timer: ResMut<FreezeBallTimer>) {
    if !freeze_ball_timer.0.paused() && freeze_ball_timer.0.tick(time.delta()).just_finished() {
        freeze_ball_timer.0.pause();
        freeze_ball_timer.0.reset();
    }
}

fn _update_ball_due_to_collision(
    ball_transform: &mut Mut<Transform>,
    ball: &mut Mut<Ball>,
    new_ball_translation: Vec3,
    new_ball_angle: f32,
) {
    {
        ball_transform.translation = new_ball_translation;
        ball.direction = Vec3::new(
            new_ball_angle.to_radians().cos(),
            new_ball_angle.to_radians().sin(),
            0.,
        );
    }
}
