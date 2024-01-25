use std::vec;

use bevy::prelude::*;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, load_assets)
            .add_systems(Update, spawn_ship);
    }
}

// cargo ships
//  container ship:     8x(2+1)x4

//  freighter:          8x(2+1)x2
//  heavy transport:    4x(2+1)x2

//  barge:              8x(2+1)
//  transport:          4x1x2
//  light transport:    4x1
//  courier:            2x1

enum CargoShip {
    Container,
    Freighter,
    HeavyTransport,
    Barge,
    Transport,
    LightTransport,
    Courier,
}

// should this be a trait?
#[derive(Resource)]
struct ContainerShip {
    mesh: Handle<Mesh>,
}

impl ContainerShip {
    fn new(mesh_handle: Handle<Mesh>) -> Self {
        ContainerShip {
            mesh: mesh_handle,
        }
    }
    fn get_mesh() -> Mesh {
        let length = 8;
        let width = 2;
        let height = 2;
        let offset = 0.1;
        let hull = CargoHull::new(Vec3::ZERO, length, width, height, offset);
        Mesh::from(hull)
    }
}

#[derive(Resource)]
struct Metal {
    material: Handle<StandardMaterial>,
}

fn load_assets(
    mut com: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let hull_mesh = ContainerShip::get_mesh();
    com.insert_resource(ContainerShip::new(meshes.add(hull_mesh)));

    let metal = StandardMaterial::from(Color::DARK_GRAY);
    com.insert_resource(Metal {
        material: materials.add(metal),
    });
}

#[derive(Component)]
pub struct SpawnShip;

fn spawn_ship(
    mut com: Commands,
    hull_module: Res<ContainerShip>,
    metal: Res<Metal>,
    q: Query<Entity, With<SpawnShip>>,
) {
    for parent in q.iter() {
        let hull = PbrBundle {
            mesh: hull_module.mesh.clone(),
            material: metal.material.clone(),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        };

        let child = com.spawn(hull).id();
        com.entity(parent).push_children(&[child]);
        com.entity(parent).remove::<SpawnShip>();

    }
}

struct Vertex {
    position: Vec3,
    normal: Vec3,
}

struct Square {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl Square {
    fn new(origin: Vec3, normal: Vec3, offset: f32) -> Self {

        if normal == Vec3::ZERO {
            panic!("Cannot find perpendicular vectors for a zero vector");
        }
    
        // Choose an arbitrary vector that is not parallel to 'v'.
        // For example, if 'v' is not parallel to the x-axis, use Vec3::X.
        let arbitrary_vector = if normal.x.abs() > 0.1 { Vec3::Y } else { Vec3::X };
    
        // First perpendicular vector
        let mut perp_1 = normal.cross(arbitrary_vector).normalize();
    
        // Second perpendicular vector
        let mut perp_2 = normal.cross(perp_1).normalize();

        perp_1 *= 0.5 - offset*0.5;
        perp_2 *= 0.5 - offset*0.5;

        let vertices = vec![
            Vertex { position: origin + (-perp_1) + (-perp_2), normal, },
            Vertex { position: origin + (perp_1) + (-perp_2), normal, },
            Vertex { position: origin + (perp_1) + (perp_2), normal, },
            Vertex { position: origin + (-perp_1) + (perp_2), normal, },
        ];

        let indices: Vec<u32> = vec![
            0, 1, 2, 2, 3, 0,
        ];

        Square {
            vertices,
            indices,
        }
    }
}

struct Cube {
    sides: Vec<Square>,
}

impl Cube {
    fn new(origin: Vec3, offset: f32) -> Self {
        let mut sides = vec![];
        for normal in vec![
            Vec3::X,
            Vec3::Y,
            Vec3::Z,
            -Vec3::X,
            -Vec3::Y,
            -Vec3::Z,
        ] {
            sides.push(Square::new(origin + (normal * (0.5 - offset*0.5)), normal, offset));
        }

        Cube {
            sides,
        }
    
    }
}

struct CargoHull {
    cubes: Vec<Cube>,
}

impl CargoHull {
    fn new(translation: Vec3, length: i32, width: i32, height: i32, offset: f32) -> CargoHull {
        let mut cubes = vec![];
        for z_offset in 0..length {
            let z = z_offset as f32;

            let mut sides: Vec<f32> = vec![];
            if width == 2 {
                sides.push(-1.0);
                sides.push(1.0);
            } else {
                sides.push(0.0);
            }

            for x_offset in sides.iter() {
                let x = *x_offset;
                for y_offset in 0..height {
                    let y = y_offset as f32;
                    let origin = Vec3::new(x, y, z) + translation;
                    cubes.push(Cube::new(origin, offset));
                }
            }
        }

        CargoHull {
            cubes,
        }
    }
}

use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

impl From<CargoHull> for Mesh {
    fn from(hull: CargoHull) -> Self {

        let mut positions: Vec<[f32; 3]> = vec![];
        let mut normals: Vec<[f32; 3]> = vec![];
        let mut indices: Vec<u32> = vec![];
        let mut indices_offset = 0;

        for cube in hull.cubes.iter() {
            for square in cube.sides.iter() {
                for vertex in square.vertices.iter() {
                    positions.push(vertex.position.into());
                    normals.push(vertex.normal.into());
                }
                for index in square.indices.iter() {
                    indices.push(index + indices_offset);
                }
                indices_offset += 4;
            }
        }

        Mesh::new(PrimitiveTopology::TriangleList)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_indices(Some(Indices::U32(indices)))
    }
}