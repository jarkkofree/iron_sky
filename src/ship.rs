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


trait MeshData {
    fn cube_size() -> f32;
}

#[derive(Resource)]
struct ShipModule<T> {
    mesh: Handle<Mesh>,
    phantom: std::marker::PhantomData<T>,
}

impl<T: MeshData> ShipModule<T> {
    fn new(meshes: &mut ResMut<Assets<Mesh>>) -> Self {
        let shape = shape::Cube::new(T::cube_size());
        let mesh = Mesh::from(shape);
        Self {
            mesh: meshes.add(mesh),
            phantom: std::marker::PhantomData,
        }
    }
}

#[derive(Resource)]
struct Cockpit;
impl MeshData for Cockpit { fn cube_size() -> f32 { 0.8 } }

#[derive(Resource)]
struct SolidsBay;
impl MeshData for SolidsBay { fn cube_size() -> f32 { 1.0 } }

#[derive(Resource)]
struct Engine;
impl MeshData for Engine { fn cube_size() -> f32 { 0.6 } }

#[derive(Resource)]
struct MiningDrill;
impl MeshData for MiningDrill { fn cube_size() -> f32 { 0.4 } }

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
    com.insert_resource(ShipModule::<Cockpit>::new(&mut meshes));
    com.insert_resource(ShipModule::<SolidsBay>::new(&mut meshes));
    com.insert_resource(ShipModule::<Engine>::new(&mut meshes));
    com.insert_resource(ShipModule::<MiningDrill>::new(&mut meshes));

    com.insert_resource(Iron::new(&mut materials));
    com.insert_resource(Steel::new(&mut materials));
    com.insert_resource(Plasma::new(&mut materials));
}

#[derive(Component)]
pub struct Spawn;

fn spawn(
    mut com: Commands,
    q: Query<Entity, With<Spawn>>,
    cockpit: Res<ShipModule<Cockpit>>,
    mining_drill: Res<ShipModule<MiningDrill>>,
    solids_bay: Res<ShipModule<SolidsBay>>,
    engine: Res<ShipModule<Engine>>,
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