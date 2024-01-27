use bevy::prelude::*;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, load)
            .add_systems(Update, (
                spawn,
            ));
    }
}

#[derive(Resource)]
struct Cockpit {
    mesh: Handle<Mesh>,
}

impl Cockpit {
    fn new(meshes: &mut ResMut<Assets<Mesh>>) -> Self {
        let shape = shape::Cube::new(0.8);
        let mesh = Mesh::from(shape);
        Self {
            mesh: meshes.add(mesh),
        }
    }
}

#[derive(Resource)]
struct SolidsBay {
    mesh: Handle<Mesh>,
}

impl SolidsBay {
    fn new(meshes: &mut ResMut<Assets<Mesh>>) -> Self {
        let shape = shape::Cube::new(1.0);
        let mesh = Mesh::from(shape);
        Self {
            mesh: meshes.add(mesh),
        }
    }
}

#[derive(Resource)]
struct Engine {
    mesh: Handle<Mesh>,
}

impl Engine {
    fn new(meshes: &mut ResMut<Assets<Mesh>>) -> Self {
        let shape = shape::Cube::new(0.6);
        let mesh = Mesh::from(shape);
        Self {
            mesh: meshes.add(mesh),
        }
    }
}

#[derive(Resource)]
struct MiningDrill {
    mesh: Handle<Mesh>,
}

impl MiningDrill {
    fn new(meshes: &mut ResMut<Assets<Mesh>>) -> Self {
        let shape = shape::Cube::new(0.4);
        let mesh = Mesh::from(shape);
        Self {
            mesh: meshes.add(mesh),
        }
    }
}

#[derive(Resource)]
struct Iron {
    material: Handle<StandardMaterial>,
}

impl Iron {
    fn new(materials: &mut ResMut<Assets<StandardMaterial>>) -> Self {
        let base_color = Color::GRAY;
        let material = StandardMaterial {
            base_color,
            perceptual_roughness: 0.8,
            metallic: 1.0,
            ..default()
        };
        Self {
            material: materials.add(material),
        }
    }
}

#[derive(Resource)]
struct Steel {
    material: Handle<StandardMaterial>,
}

impl Steel {
    fn new(materials: &mut ResMut<Assets<StandardMaterial>>) -> Self {
        let base_color = Color::SILVER;
        let material = StandardMaterial {
            base_color,
            perceptual_roughness: 0.6,
            metallic: 1.0,
            ..default()
        };
        Self {
            material: materials.add(material),
        }
    }
}

#[derive(Resource)]
struct Plasma {
    material: Handle<StandardMaterial>,
}

impl Plasma {
    fn new(materials: &mut ResMut<Assets<StandardMaterial>>) -> Self {
        let emissive = Color::RED;
        let material = StandardMaterial {
            base_color: Color::BLACK,
            emissive,
            ..default()
        };
        Self {
            material: materials.add(material),
        }
    }
}

fn load(
    mut com: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    com.insert_resource(Cockpit::new(&mut meshes));
    com.insert_resource(SolidsBay::new(&mut meshes));
    com.insert_resource(Engine::new(&mut meshes));
    com.insert_resource(MiningDrill::new(&mut meshes));

    com.insert_resource(Iron::new(&mut materials));
    com.insert_resource(Steel::new(&mut materials));
    com.insert_resource(Plasma::new(&mut materials));
}

#[derive(Component)]
pub struct Spawn;

fn spawn(
    mut com: Commands,
    q: Query<Entity, With<Spawn>>,
    cockpit: Res<Cockpit>,
    mining_drill: Res<MiningDrill>,
    solids_bay: Res<SolidsBay>,
    engine: Res<Engine>,
    iron: Res<Iron>,
    steel: Res<Steel>,
    plasma: Res<Plasma>,
) {
    for parent in q.iter() {
        let cockpit = PbrBundle {
            mesh: cockpit.mesh.clone(),
            material: steel.material.clone(),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        };
        let mining_drill = PbrBundle {
            mesh: mining_drill.mesh.clone(),
            material: steel.material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
            ..default()
        };
        let solids_bay = PbrBundle {
            mesh: solids_bay.mesh.clone(),
            material: iron.material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        };
        let engine = PbrBundle {
            mesh: engine.mesh.clone(),
            material: plasma.material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
            ..default()
        };
        let components = [cockpit, mining_drill, solids_bay, engine];
        for component in components.iter() {
            let child = com.spawn(component.clone()).id();
            com.entity(parent).push_children(&[child]);
        }
        com.entity(parent).remove::<Spawn>();
    }
}