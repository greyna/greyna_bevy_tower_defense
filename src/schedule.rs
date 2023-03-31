use bevy::prelude::*;

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(GameplaySet::Input.before(GameplaySet::Logic))
            .configure_set(GameplaySet::LogicMovement.in_set(GameplaySet::Logic))
            .configure_set(GameplaySet::LogicAction.in_set(GameplaySet::Logic))
            .configure_set(GameplaySet::LogicCollisions.in_set(GameplaySet::Logic))
            .configure_set(GameplaySet::LogicMovement.before(GameplaySet::LogicAction))
            .configure_set(GameplaySet::LogicAction.before(GameplaySet::LogicCollisions))
            .configure_set(GameplaySet::LogicCollisions.before(GameplaySet::LogicPostCollisions))
            .configure_set(GameplaySet::Logic.before(GameplaySet::Depiction));
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameplaySet {
    Input,
    Logic,
    LogicMovement,
    LogicAction,
    LogicCollisions,
    LogicPostCollisions,
    Depiction,
}
