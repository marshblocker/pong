// Pong
//
// Milestones:
//      1. Create two paddles (DONE)
//      2. Move paddles (DONE)
//      3. Add collision detection to paddles on walls
//      4. Create ball and move it in a random direction when it spawns
//      5. Add collision detection to ball on walls
//      6. Add collision detection to ball on paddles
//      7. Restart position of ball when it goes past either of the paddles' goal field

use bevy::prelude::*;

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
        .add_plugin(SetupPlugin)
        .add_system(move_paddles_system)
        .run();
}

const WINDOW_WIDTH: f32 = 600.;
const WINDOW_HEIGHT: f32 = 400.;

const WINDOW_WIDTH_HALF: f32 = WINDOW_WIDTH / 2.;
const WINDOW_HEIGHT_HALF: f32 = WINDOW_HEIGHT / 2.;

const PADDLE_WIDTH: f32 = 20.;
const PADDLE_HEIGHT: f32 = 80.;

const PADDLE_SPEED: f32 = 300.;
struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_preliminaries_system)
            .add_startup_system(spawn_paddles_system);
    }
}

#[derive(Component)]
struct LeftPaddle;

#[derive(Component)]
struct RightPaddle;

#[derive(Component)]
struct Ball;

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

fn move_paddles_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut left_paddle: Query<&mut Transform, (With<LeftPaddle>, Without<RightPaddle>)>,
    mut right_paddle: Query<&mut Transform, With<RightPaddle>>
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
