use bevy::prelude::*;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, load_environment);
    }
}

fn load_environment(
    mut com: Commands,
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
}