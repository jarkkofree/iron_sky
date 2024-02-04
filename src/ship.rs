use bevy::prelude::*;
use crate::AppState;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Menu), (
                load_miner,
            ))
            .add_systems(Update, (
                spawn_miner.run_if(in_state(AppState::Play)),
            ));
    }
}

struct Module {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    offset: Vec3,
}

#[derive(Resource)]
struct ShipModules {
    modules: Vec<Module>,
}

impl ShipModules {
    fn bundle(&self) -> Vec<PbrBundle> {
        let mut bundles = vec![];

        for module in &self.modules {
            let transform = Transform::from_translation(module.offset);
            bundles.push(PbrBundle {
                mesh: module.mesh.clone(),
                material: module.material.clone(),
                transform,
                ..Default::default()
            });
        }

        bundles
    }

}

fn load_miner(
    mut com: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let steel = StandardMaterial {
        base_color: Color::SILVER,
        perceptual_roughness: 0.5,
        metallic: 1.0,
        ..default()
    };
    let steel_handle = materials.add(steel);
    let hull_shape = Mesh::from(
        shape::Box::new(1.0, 1.0, 2.0)
    );
    let hull_handle = meshes.add(hull_shape);
    let hull_offset = Vec3::new(0.0, 0.0, 0.0);
    let hull = Module {
        mesh: hull_handle,
        material: steel_handle,
        offset: hull_offset,
    };

    let plasma = StandardMaterial {
        base_color: Color::BLACK,
        emissive: Color::RED,
        ..default()
    };
    let plasma_handle = materials.add(plasma);
    let thruster_shape = Mesh::from(
        shape::Box::new(0.5, 0.5, 0.1)
    );
    let thruster_handle = meshes.add(thruster_shape);
    let thruster_offset = Vec3::new(0.0, 0.0, 1.05);
    let thruster = Module {
        mesh: thruster_handle,
        material: plasma_handle,
        offset: thruster_offset,
    };



    let miner = ShipModules {
        modules: vec![
            hull,
            thruster,
        ],
    };
    com.insert_resource(miner);


}

#[derive(Component)]
pub struct SpawnMiner;

fn spawn_miner(
    mut com: Commands,
    q: Query<Entity, With<SpawnMiner>>,
    hull: Res<ShipModules>,
) {
    for parent in q.iter() {

        let bundles = hull.bundle();
        for bundle in bundles {
            let child = com.spawn(bundle).id();
            com.entity(parent).push_children(&[child]);
        }
        com.entity(parent).remove::<SpawnMiner>();
    }
}