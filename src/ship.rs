use bevy::prelude::*;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, load_assets)
            .add_systems(Update, (
                spawn_courier,
                spawn_light_transport,
                spawn_transport,
                spawn_barge,
                spawn_heavy_transport,
                spawn_freighter,
                spawn_container_ship,
            ));
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

trait ShipData {
    fn new() -> Self where Self: Sized;
    fn length(&self) -> i32;
    fn width(&self) -> i32;
    fn height(&self) -> i32;
}

#[derive(Component)]
pub struct SpawnCourier;

struct Courier { length: i32, width: i32, height: i32, }

impl ShipData for Courier {
    fn new() -> Self {
        Courier { length: 2, width: 1, height: 1, }
    }
    fn length(&self) -> i32 { self.length }
    fn width(&self) -> i32 { self.width }
    fn height(&self) -> i32 { self.height }
}

#[derive(Component)]
pub struct SpawnLightTransport;

struct LightTransport { length: i32, width: i32, height: i32, }

impl ShipData for LightTransport {
    fn new() -> Self {
        LightTransport { length: 4, width: 1, height: 1, }
    }
    fn length(&self) -> i32 { self.length }
    fn width(&self) -> i32 { self.width }
    fn height(&self) -> i32 { self.height }
}

#[derive(Component)]
pub struct SpawnTransport;

struct Transport { length: i32, width: i32, height: i32, }

impl ShipData for Transport {
    fn new() -> Self {
        Transport { length: 4, width: 1, height: 2, }
    }
    fn length(&self) -> i32 { self.length }
    fn width(&self) -> i32 { self.width }
    fn height(&self) -> i32 { self.height }
}

#[derive(Component)]
pub struct SpawnBarge;

struct Barge { length: i32, width: i32, height: i32, }

impl ShipData for Barge {
    fn new() -> Self {
        Barge { length: 8, width: 2, height: 1, }
    }
    fn length(&self) -> i32 { self.length }
    fn width(&self) -> i32 { self.width }
    fn height(&self) -> i32 { self.height }
}

#[derive(Component)]
pub struct SpawnHeavyTransport;

struct HeavyTransport { length: i32, width: i32, height: i32, }

impl ShipData for HeavyTransport {
    fn new() -> Self {
        HeavyTransport { length: 4, width: 2, height: 2, }
    }
    fn length(&self) -> i32 { self.length }
    fn width(&self) -> i32 { self.width }
    fn height(&self) -> i32 { self.height }
}

#[derive(Component)]
pub struct SpawnFreighter;

struct Freighter { length: i32, width: i32, height: i32, }

impl ShipData for Freighter {
    fn new() -> Self {
        Freighter { length: 8, width: 2, height: 2, }
    }
    fn length(&self) -> i32 { self.length }
    fn width(&self) -> i32 { self.width }
    fn height(&self) -> i32 { self.height }
}

#[derive(Component)]
pub struct SpawnContainerShip;

struct ContainerShip { length: i32, width: i32, height: i32, }

impl ShipData for ContainerShip {
    fn new() -> Self {
        ContainerShip { length: 8, width: 2, height: 4, }
    }
    fn length(&self) -> i32 { self.length }
    fn width(&self) -> i32 { self.width }
    fn height(&self) -> i32 { self.height }
}

#[derive(Resource)]
struct Ship<T: ShipData> {
    mesh: Handle<Mesh>,
    _data: T,
}

impl<T: ShipData> Ship<T> {
    fn new(meshes: &mut ResMut<Assets<Mesh>>) -> Self {
        let data = T::new();

        let length = data.length();
        let width = data.width();
        let height = data.height();
        let offset = 0.1;
        let hull = CargoHull::new(Vec3::ZERO, length, width, height, offset);
        let hull_mesh = Mesh::from(hull);
        let mesh = meshes.add(hull_mesh);
        Ship {
            mesh,
            _data: data,
        }
    }

    fn build_bundle(&self, metal: &Res<Metal>) -> PbrBundle {
        PbrBundle {
            mesh: self.mesh.clone(),
            material: metal.material.clone(),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        }
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
    com.insert_resource(Ship::<Courier>::new(&mut meshes));
    com.insert_resource(Ship::<LightTransport>::new(&mut meshes));
    com.insert_resource(Ship::<Transport>::new(&mut meshes));
    com.insert_resource(Ship::<Barge>::new(&mut meshes));
    com.insert_resource(Ship::<HeavyTransport>::new(&mut meshes));
    com.insert_resource(Ship::<Freighter>::new(&mut meshes));
    com.insert_resource(Ship::<ContainerShip>::new(&mut meshes));

    let metal = StandardMaterial::from(Color::DARK_GRAY);
    com.insert_resource(Metal {
        material: materials.add(metal),
    });
}

#[derive(Component)]
pub struct SpawnExample;

fn _spawn_example(
    mut com: Commands,
    ship: Res<Ship<Freighter>>,
    metal: Res<Metal>,
    q: Query<Entity, With<SpawnExample>>,
) {
    for parent in q.iter() {
        let hull = ship.build_bundle(&metal);
        let child = com.spawn(hull).id();
        com.entity(parent).push_children(&[child]);
        com.entity(parent).remove::<SpawnExample>();
    }
}

macro_rules! create_spawn_function {
    ($func_name:ident, $spawn_tag:ty, $ship_type:ty) => {
        fn $func_name(
            mut com: Commands,
            ship: Res<$ship_type>,
            metal: Res<Metal>,
            q: Query<Entity, With<$spawn_tag>>,
        ) {
            for parent in q.iter() {
                let hull = ship.build_bundle(&metal);
                let child = com.spawn(hull).id();
                com.entity(parent).push_children(&[child]);
                com.entity(parent).remove::<$spawn_tag>();
            }
        }
    };
}

create_spawn_function!(spawn_courier, SpawnCourier, Ship<Courier>);
create_spawn_function!(spawn_light_transport, SpawnLightTransport, Ship<LightTransport>);
create_spawn_function!(spawn_transport, SpawnTransport, Ship<Transport>);
create_spawn_function!(spawn_barge, SpawnBarge, Ship<Barge>);
create_spawn_function!(spawn_heavy_transport, SpawnHeavyTransport, Ship<HeavyTransport>);
create_spawn_function!(spawn_freighter, SpawnFreighter, Ship<Freighter>);
create_spawn_function!(spawn_container_ship, SpawnContainerShip, Ship<ContainerShip>);

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