// Copyright © 2023 Gabriel Kenneth L. Marinas <gabrielkennethmarinas@gmail.com>
// This work is free. You can redistribute it and/or modify it under the
// terms of the Do What The Fuck You Want To Public License, Version 2,
// as published by Sam Hocevar. See the COPYING file for more details.

use bevy::prelude::*;

use super::*;

pub type PaddleType = Or<(With<LeftPaddle>, With<RightPaddle>)>;

pub const PADDLE_WIDTH: f32 = 20.;
pub const PADDLE_HEIGHT: f32 = 80.;
pub const PADDLE_WIDTH_HALF: f32 = PADDLE_WIDTH / 2.;
pub const PADDLE_HEIGHT_HALF: f32 = PADDLE_HEIGHT / 2.;
pub const PADDLE_SPEED: f32 = 300.;

#[derive(Component)]
pub struct LeftPaddle;

#[derive(Component)]
pub struct RightPaddle;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_paddles_system)
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
        LeftPaddle,
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
        RightPaddle,
    ));
}

fn move_paddle_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut left_paddle: Query<&mut Transform, (With<LeftPaddle>, Without<RightPaddle>)>,
    mut right_paddle: Query<&mut Transform, With<RightPaddle>>,
) {
    if keyboard_input.pressed(KeyCode::W) {
        let mut left_paddle_transform = left_paddle.single_mut();
        left_paddle_transform.translation.y += PADDLE_SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::S) {
        let mut left_paddle_transform = left_paddle.single_mut();
        left_paddle_transform.translation.y -= PADDLE_SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::Up) {
        let mut right_paddle_transform = right_paddle.single_mut();
        right_paddle_transform.translation.y += PADDLE_SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::Down) {
        let mut right_paddle_transform = right_paddle.single_mut();
        right_paddle_transform.translation.y -= PADDLE_SPEED * time.delta_seconds();
    }
}

fn handle_paddle_collision_system(mut paddles_query: Query<&mut Transform, PaddleType>) {
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
