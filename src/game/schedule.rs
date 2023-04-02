use crate::AppState;
use bevy::prelude::*;

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app
            // states
            .configure_sets(
                (GameSet::Input, GameSet::Logic, GameSet::Depiction)
                    .in_set(OnUpdate(AppState::Game)),
            )
            // containers
            .configure_sets(
                (
                    GameSet::LogicMovement,
                    GameSet::LogicAction,
                    GameSet::LogicCollisions,
                )
                    .in_set(GameSet::Logic),
            )
            // dependencies
            .configure_set(GameSet::Input.before(GameSet::Logic))
            .configure_set(GameSet::LogicMovement.before(GameSet::LogicCollisions))
            .configure_set(GameSet::LogicCollisions.before(GameSet::LogicAction))
            .configure_set(GameSet::Logic.before(GameSet::Depiction));
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameSet {
    Input,
    Logic,
    LogicMovement,
    LogicAction,
    LogicCollisions,
    Depiction,
}
