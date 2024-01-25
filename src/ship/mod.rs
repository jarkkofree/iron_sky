use bevy::prelude::*;
use crate::mesh;

pub mod cargo;
pub mod passenger;

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

#[derive(Clone, Copy)]
enum HullType {
    Cargo,
    Passenger,
}

trait ShipData {
    fn new() -> Self where Self: Sized;
    fn length(&self) -> i32;
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn hull_type(&self) -> HullType;
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

        let hull_mesh;
        match data.hull_type() {
            HullType::Cargo => {
                let shape = mesh::CargoHull::new(Vec3::ZERO, length, width, height, offset);
                hull_mesh = Mesh::from(shape);
            },
            HullType::Passenger => {
                let shape = mesh::CargoHull::new(Vec3::ZERO, length, width, height, offset);
                hull_mesh = Mesh::from(shape);
            },
        }
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
    com.insert_resource(Ship::<cargo::Courier>::new(&mut meshes));
    com.insert_resource(Ship::<cargo::LightTransport>::new(&mut meshes));
    com.insert_resource(Ship::<cargo::Transport>::new(&mut meshes));
    com.insert_resource(Ship::<cargo::Barge>::new(&mut meshes));
    com.insert_resource(Ship::<cargo::HeavyTransport>::new(&mut meshes));
    com.insert_resource(Ship::<cargo::Freighter>::new(&mut meshes));
    com.insert_resource(Ship::<cargo::ContainerShip>::new(&mut meshes));

    com.insert_resource(Metal::<Iron>::new(&mut materials));
}

#[derive(Component)]
pub struct SpawnExample;

fn _spawn_example(
    mut com: Commands,
    ship: Res<Ship<cargo::Freighter>>,
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

create_spawn_function!(spawn_courier, cargo::SpawnCourier, Ship<cargo::Courier>);
create_spawn_function!(spawn_light_transport, cargo::SpawnLightTransport, Ship<cargo::LightTransport>);
create_spawn_function!(spawn_transport, cargo::SpawnTransport, Ship<cargo::Transport>);
create_spawn_function!(spawn_barge, cargo::SpawnBarge, Ship<cargo::Barge>);
create_spawn_function!(spawn_heavy_transport, cargo::SpawnHeavyTransport, Ship<cargo::HeavyTransport>);
create_spawn_function!(spawn_freighter, cargo::SpawnFreighter, Ship<cargo::Freighter>);
create_spawn_function!(spawn_container_ship, cargo::SpawnContainerShip, Ship<cargo::ContainerShip>);