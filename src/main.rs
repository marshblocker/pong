// Copyright © 2023 Gabriel Kenneth L. Marinas <gabrielkennethmarinas@gmail.com>
// This work is free. You can redistribute it and/or modify it under the
// terms of the Do What The Fuck You Want To Public License, Version 2,
// as published by Sam Hocevar. See the COPYING file for more details.

pub mod ball;
pub mod paddle;
pub mod score;

use bevy::prelude::*;

use ball::BallPlugin;
use paddle::PaddlePlugin;
use score::{GameEndEvent, ScorePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pong".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(AppPlugin)
        .add_plugin(PaddlePlugin)
        .add_plugin(BallPlugin)
        .add_plugin(ScorePlugin)
        .run();
}

const WINDOW_WIDTH: f32 = 600.;
const WINDOW_HEIGHT: f32 = 400.;
const WINDOW_WIDTH_HALF: f32 = WINDOW_WIDTH / 2.;
const WINDOW_HEIGHT_HALF: f32 = WINDOW_HEIGHT / 2.;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Ongoing,
    End,
}

struct GameRestartEvent;

struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameRestartEvent>()
            .add_state::<GameState>()
            .add_startup_system(spawn_preliminaries_system)
            .add_system(update_game_state_listener_system)
            .add_system(restart_game_system);
    }
}

fn spawn_preliminaries_system(mut commands: Commands) {
    // Spawn camera;
    commands.spawn(Camera2dBundle::default());

    // Spawn background;
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
            color: Color::BLACK,
            ..default()
        },
        ..default()
    });
}

fn update_game_state_listener_system(
    mut game_end_event_reader: EventReader<GameEndEvent>,
    mut game_restart_event_reader: EventReader<GameRestartEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if game_end_event_reader.iter().next().is_some() {
        next_state.set(GameState::End);
    }

    if game_restart_event_reader.iter().next().is_some() {
        next_state.set(GameState::Ongoing);
    }
}

fn restart_game_system(
    mut game_restart_event_writer: EventWriter<GameRestartEvent>,
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>,
) {
    if game_state.0 == GameState::End && keyboard_input.just_pressed(KeyCode::R) {
        game_restart_event_writer.send(GameRestartEvent);
    }
}