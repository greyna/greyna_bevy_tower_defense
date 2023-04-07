use super::schedule::GameSet;
use crate::AppState;
use bevy::prelude::*;

pub struct GoldPlugin;

impl Plugin for GoldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(gain_gold.in_set(GameSet::Logic))
            .add_event::<EnemyKilled>()
            .add_system(init_gold.in_schedule(OnEnter(AppState::Game)));
    }
}

pub fn gain_gold(mut enemy_killed_receiver: EventReader<EnemyKilled>, mut gold: ResMut<Gold>) {
    const GOLD_PER_ENEMY_KILL: u32 = 20;
    for _ in enemy_killed_receiver.iter() {
        gold.0 += GOLD_PER_ENEMY_KILL;

        println!(
            "Enemy kill got you {} gold. You now have {} gold.",
            GOLD_PER_ENEMY_KILL, gold.0
        );
    }
}

pub fn init_gold(mut commands: Commands) {
    let gold = Gold(1000);
    println!("You have {} gold.", gold.0);
    commands.insert_resource(gold);
}

#[derive(Resource)]
pub struct Gold(pub u32);

pub struct EnemyKilled;
