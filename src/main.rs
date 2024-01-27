use bevy::prelude::*;
use bevy_panorbit_camera;

mod environment;
mod ship;
mod asteroid;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            bevy_panorbit_camera::PanOrbitCameraPlugin,
            environment::EnvironmentPlugin,
            ship::ShipPlugin,
            asteroid::AsteroidPlugin,
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

    com.spawn((
        asteroid::Spawn,
        PbrBundle {
            transform: Transform::from_translation(Vec3::new(-5.0, 0.0, -5.0)),
            ..default()
        }
    ));
}

fn update(
    mut _com: Commands,
) {

}