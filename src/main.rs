use bevy::prelude::*;
use bevy_panorbit_camera;

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
        ))
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut com: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    com.insert_resource(ClearColor(Color::BLACK));
    com.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.01,
    });
    let extent = 10.0;
    let camera_translation = Vec3::new(extent, extent, extent);
    let camera_transform = Transform::from_translation(camera_translation)
        .looking_at(Vec3::ZERO, Vec3::Y);
    com.spawn((
        Camera3dBundle {
            transform: camera_transform,
            ..default()
        },
        bevy_panorbit_camera::PanOrbitCamera::default(),
    ));
    let light_direction = Vec3::new(-extent*0.5, -extent*1.0, -extent*0.5);
    let light_transform = Transform::IDENTITY
        .looking_at(light_direction, Vec3::Y);
    let directional_light = DirectionalLight {
        illuminance: 10_000.0,
        shadows_enabled: true,
        ..default()
    };
    com.spawn(
        DirectionalLightBundle {
            transform: light_transform,
            directional_light: directional_light,
            ..default()
        }
    );

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