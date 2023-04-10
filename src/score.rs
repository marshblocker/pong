// Copyright © 2023 Gabriel Kenneth L. Marinas <gabrielkennethmarinas@gmail.com>
// This work is free. You can redistribute it and/or modify it under the
// terms of the Do What The Fuck You Want To Public License, Version 2,
// as published by Sam Hocevar. See the COPYING file for more details.

use bevy::prelude::*;

use super::ball::GoalEvent;

const SCORE_TO_WIN: i32 = 5;

#[derive(Resource, Default)]
pub struct Score {
    left_score: i32,
    right_score: i32,
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_event::<GameEndEvent>()
            .add_system(update_score_listener_system)
            .add_system(check_for_winner_system);
    }
}

// TODO: Remove this when possible
#[allow(dead_code)]
pub struct GameEndEvent {
    final_score: (i32, i32),
}

fn update_score_listener_system(
    mut goal_event_reader: EventReader<GoalEvent>,
    mut score: ResMut<Score>,
) {
    for goal_event in goal_event_reader.iter() {
        if goal_event.left_scored {
            score.left_score += 1;
        } else {
            score.right_score += 1;
        }

        println!("Left: {}, Right: {}", score.left_score, score.right_score);
    }
}

fn check_for_winner_system(
    mut game_end_event_writer: EventWriter<GameEndEvent>,
    mut score: ResMut<Score>,
) {
    if score.left_score == SCORE_TO_WIN {
        game_end_event_writer.send(GameEndEvent {
            final_score: (score.left_score, score.right_score),
        });
        *score = Score::default();
        println!("Left player wins!");
    } else if score.right_score == SCORE_TO_WIN {
        game_end_event_writer.send(GameEndEvent {
            final_score: (score.left_score, score.right_score),
        });
        *score = Score::default();
        println!("Right player wins!");
    }
}
