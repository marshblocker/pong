// Copyright © 2023 Gabriel Kenneth L. Marinas <gabrielkennethmarinas@gmail.com>
// This work is free. You can redistribute it and/or modify it under the
// terms of the Do What The Fuck You Want To Public License, Version 2,
// as published by Sam Hocevar. See the COPYING file for more details.

// TODO: Add acceleration to paddles
// Plan: Create MIN_PADDLE_SPEED, MAX_PADDLE_SPEED, and PADDLE_ACCELERATION,
//       create a PaddleActionState enum with the following variants:
//       Active, Inactive. The paddle is active when its move button is pressed
//       and it is inactive if no move button is pressed. When the paddle is
//       active, increase its velocity by PADDLE_ACCELERATION every second until
//       it reaches MIN_PADDLE_SPEED, and do the opposite when the paddle is inactive.
//       Use a timer to measure seconds.

use bevy::prelude::*;

use super::*;

pub const PADDLE_WIDTH: f32 = 20.;
pub const PADDLE_HEIGHT: f32 = 80.;
pub const PADDLE_WIDTH_HALF: f32 = PADDLE_WIDTH / 2.;
pub const PADDLE_HEIGHT_HALF: f32 = PADDLE_HEIGHT / 2.;
pub const PADDLE_MIN_SPEED: f32 = 300.;
pub const PADDLE_MAX_SPEED: f32 = 800.;
pub const PADDLE_ACCELERATION: f32 = 50.;

#[derive(Component)]
pub struct Paddle {
    side: PaddleSide,
    speed: f32,
    direction: PaddleDirection,
}

#[derive(PartialEq, Debug)]
pub enum PaddleDirection {
    Up,
    Down,
    /// Not moving
    None,
}

#[derive(PartialEq)]
pub enum PaddleSide {
    Left,
    Right,
}

/// This timer controls the interval when the paddles' speed are changed.
#[derive(Resource)]
pub struct PaddleSpeedTimer(Timer);

impl Default for PaddleSpeedTimer {
    fn default() -> Self {
        PaddleSpeedTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PaddleSpeedTimer>()
            .add_startup_system(spawn_paddles_system)
            .add_system(handle_paddle_direction_system)
            .add_system(handle_paddle_speed_system)
            .add_system(move_paddle_system)
            .add_system(handle_paddle_collision_system.after(move_paddle_system));
    }
}

fn spawn_paddles_system(mut commands: Commands) {
    // Spawn left paddle.
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
                color: Color::WHITE,
                ..default()
            },
            transform: Transform::from_xyz(-WINDOW_WIDTH_HALF + 40., 0., 0.),
            ..default()
        },
        Paddle {
            side: PaddleSide::Left,
            speed: PADDLE_MIN_SPEED,
            direction: PaddleDirection::None,
        },
    ));

    // Spawn right paddle.
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
                color: Color::WHITE,
                ..default()
            },
            transform: Transform::from_xyz(WINDOW_WIDTH_HALF - 40., 0., 0.),
            ..default()
        },
        Paddle {
            side: PaddleSide::Right,
            speed: PADDLE_MIN_SPEED,
            direction: PaddleDirection::None,
        },
    ));
}

fn handle_paddle_direction_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut paddle_query: Query<&mut Paddle>,
) {
    let handle_paddle_direction = |mut paddle: Mut<Paddle>, up_key: KeyCode, down_key: KeyCode| {
        paddle.direction = if keyboard_input.pressed(up_key) && keyboard_input.pressed(down_key) {
            PaddleDirection::None
        } else if keyboard_input.pressed(up_key) {
            PaddleDirection::Up
        } else if keyboard_input.pressed(down_key) {
            PaddleDirection::Down
        } else {
            PaddleDirection::None
        };
    };

    for paddle in paddle_query.iter_mut() {
        match paddle.side {
            PaddleSide::Left => handle_paddle_direction(paddle, KeyCode::W, KeyCode::S),
            PaddleSide::Right => handle_paddle_direction(paddle, KeyCode::Up, KeyCode::Down),
        }
    }
}

fn handle_paddle_speed_system(
    time: Res<Time>,
    mut paddle_speed_timer: ResMut<PaddleSpeedTimer>,
    mut paddle_query: Query<&mut Paddle>,
) {
    let handle_paddle_speed = |mut paddle: Mut<Paddle>| {
        if paddle.direction == PaddleDirection::None {
            paddle.speed = paddle.speed - PADDLE_ACCELERATION;
            paddle.speed = paddle.speed.max(PADDLE_MIN_SPEED);
        } else {
            paddle.speed = paddle.speed + PADDLE_ACCELERATION;
            paddle.speed = paddle.speed.min(PADDLE_MAX_SPEED);
        }
    };

    if paddle_speed_timer.0.tick(time.delta()).just_finished() {
        for paddle in paddle_query.iter_mut() {
            handle_paddle_speed(paddle);
        }
    }
}

fn move_paddle_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut paddle_query: Query<(&mut Transform, &Paddle)>,
) {
    let move_paddle =
        |paddle: &Paddle, mut paddle_transform: Mut<Transform>, up_key: KeyCode, down_key: KeyCode| {
            if keyboard_input.pressed(up_key) {
                paddle_transform.translation.y += paddle.speed * time.delta_seconds();
            }

            if keyboard_input.pressed(down_key) {
                paddle_transform.translation.y -= paddle.speed * time.delta_seconds();
            }
        };

    for (paddle_transform, paddle) in paddle_query.iter_mut() {
        match paddle.side {
            PaddleSide::Left => move_paddle(paddle, paddle_transform, KeyCode::W, KeyCode::S),
            PaddleSide::Right => move_paddle(paddle, paddle_transform, KeyCode::Up, KeyCode::Down),
        }
    }
}

fn handle_paddle_collision_system(mut paddles_query: Query<&mut Transform, With<Paddle>>) {
    for mut paddle_transform in paddles_query.iter_mut() {
        let paddle_top = paddle_transform.translation.y + PADDLE_HEIGHT_HALF;
        let paddle_bottom = paddle_transform.translation.y - PADDLE_HEIGHT_HALF;

        if paddle_top > WINDOW_HEIGHT_HALF {
            paddle_transform.translation.y = WINDOW_HEIGHT_HALF - PADDLE_HEIGHT_HALF;
        }

        if paddle_bottom < -WINDOW_HEIGHT_HALF {
            paddle_transform.translation.y = -WINDOW_HEIGHT_HALF + PADDLE_HEIGHT_HALF;
        }
    }
}
