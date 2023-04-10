use super::{enemies::resources::Lives, schedule::GameSet};
use crate::{ui::my_window, AppState};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui::Align2, EguiContexts};

pub struct GoldPlugin;

impl Plugin for GoldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((gain_gold, gain_score).in_set(GameSet::Logic))
            .add_system(ui.in_set(GameSet::Depiction))
            .add_event::<EnemyKilled>()
            .add_system(init_gold.in_schedule(OnEnter(AppState::Game)))
            .init_resource::<Score>()
            .add_system(set_high_score.in_schedule(OnExit(AppState::Game)));
    }
}

pub fn ui(
    gold: Res<Gold>,
    lives: Res<Lives>,
    score: Res<Score>,
    mut contexts: EguiContexts,
    main_window: Query<&Window, With<PrimaryWindow>>,
) {
    const LATERAL_OFFSET: f32 = 115.0;

    my_window("Gold", Vec2::default(), &main_window)
        .anchor(Align2::CENTER_TOP, [-LATERAL_OFFSET, 10.0])
        .show(contexts.ctx_mut(), |ui| {
            ui.heading(format!("Gold: {}", gold.0));
        });

    my_window("Lives", Vec2::default(), &main_window)
        .anchor(Align2::CENTER_TOP, [-7.5, 10.0])
        .show(contexts.ctx_mut(), |ui| {
            ui.heading(format!("Lives: {}", lives.0));
        });

    my_window("Score", Vec2::default(), &main_window)
        .anchor(Align2::CENTER_TOP, [LATERAL_OFFSET, 10.0])
        .show(contexts.ctx_mut(), |ui| {
            ui.heading(format!("Score: {}", score.score));
        });
}

pub fn gain_gold(mut enemy_killed_receiver: EventReader<EnemyKilled>, mut gold: ResMut<Gold>) {
    const GOLD_PER_ENEMY_KILL: u32 = 20;
    for _ in enemy_killed_receiver.iter() {
        gold.0 += GOLD_PER_ENEMY_KILL;
    }
}

pub fn gain_score(mut enemy_killed_receiver: EventReader<EnemyKilled>, mut score: ResMut<Score>) {
    const SCORE_PER_ENEMY_KILL: i32 = 10;
    for _ in enemy_killed_receiver.iter() {
        score.score += SCORE_PER_ENEMY_KILL;
    }
}

pub fn init_gold(mut commands: Commands) {
    let gold = Gold(2500);
    println!("You have {} gold.", gold.0);
    commands.insert_resource(gold);
}

pub fn set_high_score(mut score: ResMut<Score>) {
    if score.score > score.high_score {
        score.high_score = score.score;
    }
    score.score = 0;
}

#[derive(Resource)]
pub struct Gold(pub u32);

#[derive(Resource, Default)]
pub struct Score {
    pub score: i32,
    pub high_score: i32,
}

pub struct EnemyKilled;
