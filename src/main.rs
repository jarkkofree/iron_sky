use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

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
            PanOrbitCameraPlugin,
        ))

        .add_state::<AppState>()

        .register_type::<Camera>()
        .register_type::<Light>()
        .register_type::<Cube>()
        .register_type::<CubeMesh>()
        .register_type::<MetalMaterials>()

        .add_systems(OnEnter(AppState::Load), (
            setup_test,
        ))

        .add_systems(OnEnter(AppState::Play), (
            run_new_test,
        ))

        .add_systems(Update, (
            save_test.run_if(in_state(AppState::Pause)),
        ))

        .run();
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct Camera;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct Light;


#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct CubeMesh {
    mesh: Mesh,
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
}

impl Default for CubeMesh {
    fn default() -> Self {
        let cube = shape::Cube::new(1.0);
        let mesh = Mesh::from(cube);
        let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
        let normals = mesh.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap();
        CubeMesh {
            mesh: mesh.clone(),
            positions: positions.as_float3().unwrap().to_vec(),
            normals: normals.as_float3().unwrap().to_vec(),
        }
    }
}

#[derive(Resource)]
struct CubeMeshHandle {
    handle: Handle<Mesh>,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct Cube;



#[derive(Resource)]
struct AluminumHandle {
    handle: Handle<StandardMaterial>,
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct MetalMaterials {
    aluminum: StandardMaterial,
}

impl Default for MetalMaterials {
    fn default() -> Self {
        let metallic = StandardMaterial {
            metallic: 1.0,
            perceptual_roughness: 0.5,
            ..default()
        };
        let aluminum_color = Color::hex("faf5f5").unwrap();
        let aluminum_material = StandardMaterial {
            base_color: aluminum_color,
            ..metallic
        };
        MetalMaterials {
            aluminum: aluminum_material,
        }
    }
}

fn setup_test (
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
    let metals = MetalMaterials::default();
    let aluminum_handle = materials.add(metals.aluminum.clone());
    com.insert_resource(AluminumHandle {
        handle: aluminum_handle.clone(),
    });
    com.insert_resource(metals);

    // Meshes
    let cube_mesh = CubeMesh::default();
    let cube_mesh_handle = meshes.add(cube_mesh.mesh.clone());
    com.insert_resource(cube_mesh);
    com.insert_resource(CubeMeshHandle {
        handle: cube_mesh_handle.clone(),
    });

    next_state.set(AppState::Play);
}

fn run_new_test(
    mut com: Commands,
    aluminum: Res<AluminumHandle>,
    cube_mesh: Res<CubeMeshHandle>,
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
        Camera,
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
        Light,
    ));

    let aluminum_cube = PbrBundle {
        mesh: cube_mesh.handle.clone(),
        material: aluminum.handle.clone(),
        ..Default::default()
    };

    com.spawn((
        aluminum_cube,
        Cube,
    ));

    next_state.set(AppState::Pause);
}

use bevy::tasks::IoTaskPool;
use std::{fs::File, io::Write};
fn save_test(
    world: &mut World,
) {
    let mut entity_query = world.query_filtered::<Entity, With<Transform>>();

    let entities = DynamicSceneBuilder::from_world(world)
        .allow::<Cube>()
        .allow::<Camera>()
        .allow::<Light>()
        .allow::<Transform>()
        .extract_entities(entity_query.iter(&world))
        .build();
    let resources = DynamicSceneBuilder::from_world(world)
        .allow_resource::<ClearColor>()
        .allow_resource::<AmbientLight>()
        .allow_resource::<CubeMesh>()
        .allow_resource::<MetalMaterials>()
        .extract_resources()
        .build();
    let type_registry = world.resource::<AppTypeRegistry>();
    let serialized_entities = entities.serialize_ron(type_registry).unwrap();
    let entities_filepath = String::from("save.ron");
    let serialized_resources = resources.serialize_ron(type_registry).unwrap();
    let resources_filepath = String::from("resources.ron");


    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(async move {
            // Write the scene RON data to file
            File::create(format!("assets/data/{entities_filepath}"))
                .and_then(|mut file| file.write(serialized_entities.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();

    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(async move {
            // Write the scene RON data to file
            File::create(format!("assets/data/{resources_filepath}"))
                .and_then(|mut file| file.write(serialized_resources.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
}