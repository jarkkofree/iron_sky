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


trait ModuleMeshData {
    fn cube_size() -> f32;
}

#[derive(Resource)]
struct ShipModule<T> {
    mesh: Handle<Mesh>,
    phantom: std::marker::PhantomData<T>,
}

impl<T: ModuleMeshData> ShipModule<T> {
    fn new(meshes: &mut ResMut<Assets<Mesh>>) -> Self {
        let shape = shape::Cube::new(T::cube_size());
        let mesh = Mesh::from(shape);
        Self {
            mesh: meshes.add(mesh),
            phantom: std::marker::PhantomData,
        }
    }
}

struct Cockpit;
impl ModuleMeshData for Cockpit { fn cube_size() -> f32 { 0.8 } }

struct SolidsBay;
impl ModuleMeshData for SolidsBay { fn cube_size() -> f32 { 1.0 } }

struct Engine;
impl ModuleMeshData for Engine { fn cube_size() -> f32 { 0.6 } }

struct MiningDrill;
impl ModuleMeshData for MiningDrill { fn cube_size() -> f32 { 0.4 } }


enum ModuleMaterialType {
    Metal {
        color: Color,
        roughness: f32,
    },
    Plasma {
        color: Color,
    },
}

trait ModuleMaterialData {
    fn material() -> ModuleMaterialType;
}

#[derive(Resource)]
struct ModuleMaterial<T> {
    material: Handle<StandardMaterial>,
    phantom: std::marker::PhantomData<T>,
}

impl<T: ModuleMaterialData> ModuleMaterial<T> {
    fn new(materials: &mut ResMut<Assets<StandardMaterial>>) -> Self {
        let data = T::material();
        let material;
        match data {
            ModuleMaterialType::Metal { color, roughness } => {
                material = StandardMaterial {
                    base_color: color,
                    perceptual_roughness: roughness,
                    metallic: 1.0,
                    ..default()
                };

            },
            ModuleMaterialType::Plasma { color } => {
                material = StandardMaterial {
                    base_color: Color::BLACK,
                    emissive: color,
                    ..default()
                };
            },
        }
        Self {
            material: materials.add(material),
            phantom: std::marker::PhantomData,
        }
    }
}

struct Iron;

impl ModuleMaterialData for Iron {
    fn material() -> ModuleMaterialType {
        ModuleMaterialType::Metal { color: Color::GRAY, roughness: 0.8, }
    }
}

struct Steel;

impl ModuleMaterialData for Steel {
    fn material() -> ModuleMaterialType {
        ModuleMaterialType::Metal { color: Color::SILVER, roughness: 0.6, }
    }
}

struct Plasma;

impl ModuleMaterialData for Plasma {
    fn material() -> ModuleMaterialType {
        ModuleMaterialType::Plasma { color: Color::RED, }
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

    com.insert_resource(ModuleMaterial::<Iron>::new(&mut materials));
    com.insert_resource(ModuleMaterial::<Steel>::new(&mut materials));
    com.insert_resource(ModuleMaterial::<Plasma>::new(&mut materials));
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
    iron: Res<ModuleMaterial<Iron>>,
    steel: Res<ModuleMaterial<Steel>>,
    plasma: Res<ModuleMaterial<Plasma>>,
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