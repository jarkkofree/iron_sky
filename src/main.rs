use bevy::prelude::*;
use bevy_panorbit_camera;

mod environment;
mod ship;
mod mesh;

// s: 1x1x1
// m: 3x3x3
// l: 9x9x9

// ship generator, backbone, rib for 2-wide, bridge, engine on mech block
// passenger ship, single block/no gaps, forward bridge
// mining ship, like cargo ship but first block is a drill

// station plugin
// no transparency, just use emmissive for bay door, inverted cube for interior

// stations/capitals
// landing pads (m?, l?)

// combat ships? armor?
// kinetic: hull/missile
// energy: shield/plasma
// thermal: radiator/laser

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            bevy_panorbit_camera::PanOrbitCameraPlugin,
            environment::EnvironmentPlugin,
            ship::ShipPlugin,
        ))
        .add_systems(Startup, startup)
        .run();
}



fn startup(
    mut com: Commands,
) {
    com.spawn((
        PbrBundle {
            transform: Transform::from_translation(
                Vec3::new(0.0, 0.0, 0.0)
            ),
            ..default()
        },
        ship::cargo::SpawnCourier
    ));

    com.spawn((
        PbrBundle {
            transform: Transform::from_translation(
                Vec3::new(5.0, 0.0, 0.0)
            ),
            ..default()
        },
        ship::cargo::SpawnLightTransport
    ));

    com.spawn((
        PbrBundle {
            transform: Transform::from_translation(
                Vec3::new(10.0, 0.0, 0.0)
            ),
            ..default()
        },
        ship::cargo::SpawnTransport
    ));

    com.spawn((
        PbrBundle {
            transform: Transform::from_translation(
                Vec3::new(15.0, 0.0, 0.0)
            ),
            ..default()
        },
        ship::cargo::SpawnBarge
    ));

    com.spawn((
        PbrBundle {
            transform: Transform::from_translation(
                Vec3::new(20.0, 0.0, 0.0)
            ),
            ..default()
        },
        ship::cargo::SpawnHeavyTransport
    ));

    com.spawn((
        PbrBundle {
            transform: Transform::from_translation(
                Vec3::new(25.0, 0.0, 0.0)
            ),
            ..default()
        },
        ship::cargo::SpawnFreighter
    ));

    com.spawn((
        PbrBundle {
            transform: Transform::from_translation(
                Vec3::new(30.0, 0.0, 0.0)
            ),
            ..default()
        },
        ship::cargo::SpawnContainerShip
    ));

}