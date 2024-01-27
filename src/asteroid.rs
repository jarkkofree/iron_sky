use bevy::prelude::*;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, load)
            .add_systems(Update, (
                spawn,
            ));
    }
}

#[derive(Resource)]
struct Asteroid {
    mesh: Handle<Mesh>,
}

impl Asteroid {
    fn new(meshes: &mut ResMut<Assets<Mesh>>) -> Self {
        let shape = shape::Icosphere {
            radius: 1.0,
            subdivisions: 0,
        };
        let mesh = Mesh::try_from(shape).unwrap();
        Self {
            mesh: meshes.add(mesh),
        }
    }
}

#[derive(Resource)]
struct Ore {
    material: Handle<StandardMaterial>,
}

impl Ore {
    fn new(materials: &mut ResMut<Assets<StandardMaterial>>) -> Self {
        let base_color = Color::DARK_GRAY;
        let material = StandardMaterial {
            base_color,
            reflectance: 0.1,
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
    com.insert_resource(Asteroid::new(&mut meshes));

    com.insert_resource(Ore::new(&mut materials));
}

#[derive(Component)]
pub struct Spawn;

fn spawn(
    mut com: Commands,
    q: Query<Entity, With<Spawn>>,
    asteroid: Res<Asteroid>,
    ore: Res<Ore>,
) {
    for parent in q.iter() {
        let ore_asteroid = PbrBundle {
            mesh: asteroid.mesh.clone(),
            material: ore.material.clone(),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        };
        let components = [ore_asteroid];
        for component in components.iter() {
            let child = com.spawn(component.clone()).id();
            com.entity(parent).push_children(&[child]);
        }
        com.entity(parent).remove::<Spawn>();
    }
}