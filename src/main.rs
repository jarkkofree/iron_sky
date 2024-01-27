use bevy::prelude::*;
use bevy_panorbit_camera;

mod environment;

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
) {

}