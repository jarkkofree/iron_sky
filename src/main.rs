use bevy::prelude::*;
use bevy_panorbit_camera;

mod environment;
mod ship;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            environment::EnvironmentPlugin,
            bevy_panorbit_camera::PanOrbitCameraPlugin,
        ))
        .add_systems(Startup, (
            startup,
        ))
        .add_systems(Update, (
            update,
        ))
        .run();
}

/*
    asteroid
        ore
        silicon
        rare metal
        ice

    asteroid_belt

    npc
        command (captian)
        controls (pilot)
        nav (navigator)
        console (engineer)

*/


fn startup(
    mut com: Commands,
) {
    com.spawn((
        ship::Spawn,
        PbrBundle {
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        }
    ));
}

fn update(
    mut com: Commands,
) {

}