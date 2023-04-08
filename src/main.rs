// Pong
//
// Milestones:
//      1. Create two paddles (DONE)
//      2. Move paddles (DONE)
//      3. Add collision detection to paddles on walls (DONE)
//      4. Create ball and move it in a random direction when it spawns (DONE)
//      5. Add collision detection to ball on walls
//      6. Add collision detection to ball on paddles
//      7. Restart position of ball when it goes past either of the paddles' goal field

use bevy::prelude::*;
use rand::prelude::*;

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
        .add_plugin(PaddlePlugin)
        .add_plugin(BallPlugin)
        .run();
}

const WINDOW_WIDTH: f32 = 600.;
const WINDOW_HEIGHT: f32 = 400.;
const WINDOW_WIDTH_HALF: f32 = WINDOW_WIDTH / 2.;
const WINDOW_HEIGHT_HALF: f32 = WINDOW_HEIGHT / 2.;

const PADDLE_WIDTH: f32 = 20.;
const PADDLE_HEIGHT: f32 = 80.;
const PADDLE_WIDTH_HALF: f32 = PADDLE_WIDTH / 2.;
const PADDLE_HEIGHT_HALF: f32 = PADDLE_HEIGHT / 2.;
const PADDLE_SPEED: f32 = 300.;

const BALL_SIZE: f32 = 30.;
const BALL_SPEED: f32 = 300.;

struct SetupPlugin;
struct PaddlePlugin;

struct BallPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_preliminaries_system)
            .add_startup_system(spawn_paddles_system)
            .add_startup_system(spawn_ball_system);
    }
}

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_paddle_system)
            .add_system(handle_paddle_collision_system);
    }
}

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_ball_system);
    }
}

#[derive(Component)]
struct LeftPaddle;

#[derive(Component)]
struct RightPaddle;

#[derive(Component)]
struct Ball {
    direction: Vec3,
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

fn spawn_ball_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn ball
    let mut rng = rand::thread_rng();

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/ball.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
                ..default()
            },
            ..default()
        },
        Ball {
            direction: Vec3::new(
                rng.gen_range(-1000.0_f32..=1000.0_f32),
                rng.gen_range(-1000.0_f32..=1000.0_f32),
                0.,
            )
            .try_normalize()
            .unwrap_or_else(|| Vec3::new(1., 0., 0.)),
        },
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

fn handle_paddle_collision_system(
    mut paddles_query: Query<&mut Transform, Or<(With<LeftPaddle>, With<RightPaddle>)>>,
) {
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

fn move_ball_system(time: Res<Time>, mut ball_query: Query<(&mut Transform, &Ball)>) {
    let (mut ball_transform, ball) = ball_query.single_mut();
    ball_transform.translation += BALL_SPEED * ball.direction * time.delta_seconds();
}
