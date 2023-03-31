mod schedule;
mod turret;
mod utils;

use std::time::Duration;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use schedule::GameplaySet;
use turret::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TurretPlugin)
        .add_startup_systems((spawn_player, spawn_camera, spawn_target))
        .add_system(exit_game)
        .add_system(target_cursor.in_set(GameplaySet::Input))
        .add_system(move_player.in_set(GameplaySet::LogicMovement))
        .add_system(check_collisions.in_set(GameplaySet::LogicCollisions))
        .add_system(handle_collisions.in_set(GameplaySet::LogicPostCollisions))
        .add_systems(
            (first_blink.before(blink), handle_blink_requests, blink)
                .in_set(GameplaySet::Depiction),
        )
        .add_event::<Collision>()
        .run();
}

#[derive(Component)]
pub struct Player {}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/player.png"),
            ..default()
        },
        Player {},
        Collidable {},
    ));
}

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_camera(mut commands: Commands, window_q: Query<&Window, With<PrimaryWindow>>) {
    let window = window_q.single();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        MainCamera,
    ));
}

#[derive(Resource, Default)]
pub struct Target {
    pos: Option<Vec2>,
}

pub fn spawn_target(mut commands: Commands) {
    commands.init_resource::<Target>();
}

pub fn target_cursor(
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut target: ResMut<Target>,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = window_q.single();

    target.pos = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate());
}

pub fn move_player(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut player_q: Query<&mut Transform, With<Player>>,
    target: Res<Target>,
) {
    let window = window_q.single();

    let target = match target.pos {
        Some(pos) => pos,
        None => Vec2::new(window.width() / 2.0, window.height() / 2.0),
    };

    for mut player_transform in &mut player_q {
        player_transform.translation.x = target.x;
        player_transform.translation.y = target.y;
    }
}

pub fn exit_game(input: Res<Input<KeyCode>>, mut app_exit_sender: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        app_exit_sender.send(AppExit);
    }
}

#[derive(Component)]
pub struct Collidable {}

pub struct Collision {
    entity_a: Entity,
    entity_b: Entity,
}

pub fn check_collisions(
    collidables: Query<(Entity, &Transform), With<Collidable>>,
    mut collision_sender: EventWriter<Collision>,
) {
    let mut collidables_couples = collidables.iter_combinations();
    while let Some([a, b]) = collidables_couples.fetch_next() {
        if a.1.translation.distance(b.1.translation) < 64.0 {
            collision_sender.send(Collision {
                entity_a: a.0,
                entity_b: b.0,
            });
        }
    }
}

pub fn handle_collisions(mut commands: Commands, mut collisions_receiver: EventReader<Collision>) {
    for collision in collisions_receiver.iter() {
        commands.entity(collision.entity_a).insert(BlinkRequest {});
        commands.entity(collision.entity_b).insert(BlinkRequest {});
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct BlinkRequest {}

pub fn handle_blink_requests(
    mut commands: Commands,
    mut requests: Query<(Entity, Option<&mut Blinking>), With<BlinkRequest>>,
) {
    for (entity, blinking) in requests.iter_mut() {
        match blinking {
            Some(mut b) => b.reset_expiration(),
            None => drop(commands.entity(entity).insert(Blinking::default())),
        };
        commands.entity(entity).remove::<BlinkRequest>();
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Blinking {
    blink_timer: Timer,
    expiration_timer: Timer,
    first_visibility: Visibility,
}

impl Default for Blinking {
    fn default() -> Self {
        Self {
            blink_timer: Timer::from_seconds(0.12, TimerMode::Repeating),
            expiration_timer: Timer::from_seconds(0.6, TimerMode::Once),
            first_visibility: Default::default(),
        }
    }
}

impl Blinking {
    pub fn init_first_visibility(&mut self, visibility: Visibility) {
        self.first_visibility = visibility;
    }

    pub fn tick(&mut self, delta: Duration) {
        self.blink_timer.tick(delta);
        self.expiration_timer.tick(delta);
    }

    pub fn toggling_visibility(&self) -> bool {
        self.blink_timer.finished()
    }

    pub fn expired(&self) -> bool {
        self.expiration_timer.finished()
    }

    pub fn reset_expiration(&mut self) {
        self.expiration_timer.reset();
    }

    pub fn opposite_first_visibility(&self) -> Visibility {
        match self.first_visibility {
            Visibility::Inherited => Visibility::Hidden,
            Visibility::Hidden => Visibility::Inherited,
            Visibility::Visible => Visibility::Hidden,
        }
    }

    pub fn first_visibility(&self) -> Visibility {
        self.first_visibility
    }
}

pub fn first_blink(mut blinkers: Query<(&mut Visibility, &mut Blinking), Added<Blinking>>) {
    for (mut vis, mut blink) in blinkers.iter_mut() {
        blink.init_first_visibility(*vis);
        *vis = blink.opposite_first_visibility();
    }
}

pub fn blink(
    mut commands: Commands,
    time: Res<Time>,
    mut blinkers: Query<(Entity, &mut Visibility, &mut Blinking)>,
) {
    for (entity, mut vis, mut blink) in blinkers.iter_mut() {
        blink.tick(time.delta());

        if blink.expired() {
            *vis = blink.first_visibility();
            commands.entity(entity).remove::<Blinking>();
        } else if blink.toggling_visibility() {
            *vis = if *vis == blink.first_visibility() {
                blink.opposite_first_visibility()
            } else {
                blink.first_visibility()
            };
        }
    }
}
