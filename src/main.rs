use bevy::prelude::*;
use bevy_panorbit_camera;

mod environment;
mod ship;
mod asteroid;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    // Load game resources
    Load,

    // Play {New, Resume}, Settings, Editor, Exit
    Menu,

    // Game loop, Pause, Save, Quit
    Play,
    Pause,

    // Material, Mesh, Module, Ship, Station, Map, and Actor editor
    Editor,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            bevy_panorbit_camera::PanOrbitCameraPlugin,
            environment::EnvironmentPlugin,
            ship::ShipPlugin,
            asteroid::AsteroidPlugin,
        ))

        .add_state::<AppState>()

        .add_systems(OnEnter(AppState::Play), (
            startup,
        ))
        .add_systems(Update, (
            update.run_if(in_state(AppState::Play)),
        ))
        .run();
}

#[derive(Component)]
struct Miner;

#[derive(Component)]
struct Velocity(f32);

fn startup(
    mut com: Commands,
) {
    com.spawn((
        ship::SpawnMiner,
        Miner,
        Velocity(0.1),
        PbrBundle {
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        }
    ));

    com.spawn((
        asteroid::Spawn,
        PbrBundle {
            transform: Transform::from_translation(Vec3::new(-10.0, 0.0, -10.0)),
            ..default()
        }
    ));
}

fn update(
    mut _com: Commands,
    mut q: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
) {
    for (mut t, v) in q.iter_mut() {
        let forward = t.forward();
        t.translation += forward * v.0 * time.delta_seconds();
    }
}