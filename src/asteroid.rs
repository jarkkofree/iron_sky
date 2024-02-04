use bevy::prelude::*;
use rand::prelude::*;
use crate::AppState;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Menu), load)
            .add_systems(OnEnter(AppState::Play), (
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

struct AsteroidBelt {
    asteroids: Vec<Transform>,
}

impl AsteroidBelt {
    fn new() -> Self {

        let mut rng = rand::thread_rng();
        let position_range = -10..10;
        let asteroid_count = 10;

        let mut asteroids = Vec::new();
        for _ in 0..asteroid_count {
            let x = rng.gen_range(position_range.clone());
            let y = rng.gen_range(position_range.clone());
            let z = rng.gen_range(position_range.clone());
            let translation = Vec3::new(x as f32, y as f32, z as f32);
            let transform = Transform::from_translation(translation);
            asteroids.push(transform);
        }

        Self {
            asteroids,
        }
    }
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
        let asteroid_belt = AsteroidBelt::new();
        for transform in asteroid_belt.asteroids.iter() {
            let asteroid = PbrBundle {
                mesh: asteroid.mesh.clone(),
                material: ore.material.clone(),
                transform: transform.clone(),
                ..default()
            };
            let child = com.spawn(asteroid).id();
            com.entity(parent).push_children(&[child]);
        }
        com.entity(parent).remove::<Spawn>();
    }
}