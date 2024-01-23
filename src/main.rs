use bevy::prelude::*;
use bevy_panorbit_camera;

mod environment;

// s: 1x1x1
// m: 3x3x3
// l: 5x5x5

// kinetic: hull/missile
// energy: shield/plasma
// thermal: radiator/laser

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            bevy_panorbit_camera::PanOrbitCameraPlugin,
            environment::EnvironmentPlugin,
        ))
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut com: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube = shape::Cube::new(1.0);
    let cube_mesh = meshes.add(cube.into());
    let metal = StandardMaterial::from(Color::DARK_GRAY);
    let cube_material = materials.add(metal);
    com.spawn(PbrBundle {
        mesh: cube_mesh,
        material: cube_material,
        transform: Transform::from_translation(Vec3::ZERO),
        ..default()
    });
}