use bevy::prelude::*;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct MainCamera;

#[derive(Resource, Default)]
pub struct Target {
    pub pos: Option<Vec2>,
}
