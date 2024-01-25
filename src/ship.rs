use bevy::prelude::*;

use crate::mesh;

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
        let hull = mesh::CargoHull::new(Vec3::ZERO, length, width, height, offset);
        let hull_mesh = Mesh::from(hull);
        let mesh = meshes.add(hull_mesh);
        Ship {
            mesh,
            _data: data,
        }
    }

    fn build_bundle(&self, metal: &Res<Metal<Iron>>) -> PbrBundle {
        PbrBundle {
            mesh: self.mesh.clone(),
            material: metal.material.clone(),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        }
    }
}

trait MetalData {
    fn new() -> Self where Self: Sized;
    fn color(&self) -> Color;
}

struct Iron { color: Color }

impl MetalData for Iron {
    fn new() -> Self { Iron { color: Color::DARK_GRAY } }
    fn color(&self) -> Color { self.color }
}

#[derive(Resource)]
struct Metal<T: MetalData> {
    material: Handle<StandardMaterial>,
    _data: T,
}

impl<T: MetalData> Metal<T> {
    fn new(materials: &mut ResMut<Assets<StandardMaterial>>) -> Self {
        let data = T::new();
        let metal = StandardMaterial::from(data.color());
        let material = materials.add(metal);
        Metal {
            material,
            _data: data,
        }
    }

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

    com.insert_resource(Metal::<Iron>::new(&mut materials));
}

#[derive(Component)]
pub struct SpawnExample;

fn _spawn_example(
    mut com: Commands,
    ship: Res<Ship<Freighter>>,
    metal: Res<Metal<Iron>>,
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
            metal: Res<Metal<Iron>>,
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