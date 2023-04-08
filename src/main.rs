// Pong
//
// Milestones:
//      1. Create two paddles (DONE)
//      2. Move paddles (DONE)
//      3. Add collision detection to paddles on walls (DONE)
//      4. Create ball and move it in a random direction when it spawns (DONE)
//      5. Add collision detection to ball on upper and lower walls (DONE)
//      6. Add collision detection to ball on paddles (DONE)
//      7. Restart position of ball when it goes past either of the paddles' goal field (DONE)
//      8. Refactor code (DONE)
//      9. Add scoring system (DONE)
//      10. Temporarily freeze ball every start of a round (DONE)
//      11. Allow game restart
//      12. Add dynamics to gameplay -- e.g. implement acceleration
//      12. Create game menu -- vs Human or vs AI
//      13. Implement AI

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
        .add_state::<GameState>()
        .add_startup_system(spawn_preliminaries_system)
        .add_system(update_game_state_listener_system)
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
    mut next_state: ResMut<NextState<GameState>>,
) {
    if game_end_event_reader.iter().next().is_some() {
        next_state.set(GameState::End);
    }
}
