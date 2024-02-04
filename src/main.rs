use bevy::prelude::*;
use bevy_panorbit_camera;

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
        ))

        .add_state::<AppState>()

        .run();
}