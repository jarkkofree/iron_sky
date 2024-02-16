use bevy::prelude::*;
use bevy_panorbit_camera;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
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

        .add_systems(OnEnter(AppState::Load), (
            insert_resources,
        ))

        .add_systems(OnEnter(AppState::Play), (
            spawn,
        ))

        .run();
}


fn metal(name: &str) -> StandardMaterial {
    let color = match name {
        "silver" => Color::hex("faf9f5"),
        "aluminum" => Color::hex("faf5f5"),
        "platnum" => Color::hex("d6d1c8"),
        "iron" => Color::hex("c0bdba"),
        "titanium" => Color::hex("cec8c2"),
        "copper" => Color::hex("fbd8b8"),
        "gold" => Color::hex("fedc9d"),
        "brass" => Color::hex("f4e4ad"),
        _ => Ok(Color::WHITE),
    };
    StandardMaterial {
        base_color: color.unwrap(),
        metallic: 1.0,
        perceptual_roughness: 0.5,
        ..default()
    }
}

static METALS: [(&str, &str); 8] = [
    ("silver","faf9f5"),
    ("aluminum","faf5f5"),
    ("platnum","d6d1c8"),
    ("iron","c0bdba"),
    ("titanium","cec8c2"),
    ("copper","fbd8b8"),
    ("gold","fedc9d"),
    ("brass","f4e4ad"),
];

#[derive(Resource)]
struct MetalHandles {
    silver: Handle<StandardMaterial>,
    aluminum: Handle<StandardMaterial>,
    platnum: Handle<StandardMaterial>,
    iron: Handle<StandardMaterial>,
    titanium: Handle<StandardMaterial>,
    copper: Handle<StandardMaterial>,
    gold: Handle<StandardMaterial>,
    brass: Handle<StandardMaterial>,
}

fn insert_resources (
    mut com: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    com.insert_resource(ClearColor(Color::BLACK));
    com.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.01,
    });

    // Materials
    com.insert_resource(metal("silver"));

    next_state.set(AppState::Play);
}

fn spawn(
    mut com: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_state: ResMut<NextState<AppState>>
) {
    // Camera
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

    // Light
    let light_direction = Vec3::new(-extent*0.5, -extent*1.0, -extent*0.5);
    let light_transform = Transform::IDENTITY
        .looking_at(light_direction, Vec3::Y);
    let directional_light = DirectionalLight {
        illuminance: 10_000.0,
        shadows_enabled: true,
        ..default()
    };
    com.spawn((
        DirectionalLightBundle {
            transform: light_transform,
            directional_light: directional_light,
            ..default()
        },
    ));

    let silver = StandardMaterial {
        base_color: Color::hex("faf9f5").unwrap(),
        metallic: 1.0,
        perceptual_roughness: 0.5,
        ..default()
    };

    let aluminum_cube = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
        material: materials.add(silver),
        ..Default::default()
    };

    com.spawn((
        aluminum_cube,
    ));

    next_state.set(AppState::Pause);
}

// use bevy::tasks::IoTaskPool;
// use std::{fs::File, io::Write};
// fn save_test(
//     world: &mut World,
// ) {
//     let mut entity_query = world.query_filtered::<Entity, With<Transform>>();

//     let scene = DynamicSceneBuilder::from_world(world)
//         .allow::<Cube>()
//         .allow::<PlayerView>()
//         .allow::<Sunlight>()
//         .extract_entities(entity_query.iter(&world))
//         .build();
//     let type_registry = world.resource::<AppTypeRegistry>();
//     let serialized_scene = scene.serialize_ron(type_registry).unwrap();
//     let scene_filepath = String::from("save.scn.ron");


//     #[cfg(not(target_arch = "wasm32"))]
//     IoTaskPool::get()
//         .spawn(async move {
//             // Write the scene RON data to file
//             File::create(format!("assets/data/{scene_filepath}"))
//                 .and_then(|mut file| file.write(serialized_scene.as_bytes()))
//                 .expect("Error while writing scene to file");
//         })
//         .detach();
// }

// fn load_test(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let entities_filepath = String::from("save.scn.ron");
//     // "Spawning" a scene bundle creates a new entity and spawns new instances
//     // of the given scene's entities as children of that entity.
//     commands.spawn(DynamicSceneBundle {
//         // Scenes are loaded just like any other asset.
//         scene: asset_server.load(format!("data/{entities_filepath}")),
//         ..default()
//     });
// }