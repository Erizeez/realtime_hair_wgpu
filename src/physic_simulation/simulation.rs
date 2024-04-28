use bevy::{
    asset::Assets,
    ecs::system::{Commands, ResMut},
    log::info,
    math::{
        primitives::{Cuboid, Cylinder},
        Vec3,
    },
    pbr::{PbrBundle, StandardMaterial},
    prelude::{default, SpatialBundle},
    render::{color::Color, mesh::Mesh, view::NoFrustumCulling},
    transform::components::Transform,
};

use crate::plugins::instanced_mesh::{InstanceData, InstanceMaterialData};

use super::{
    data::{SimulationObject, SimulationTaskInterface},
    PhsicaSimulationScheduler,
};

pub fn init_simulation(
    scheduler: &mut PhsicaSimulationScheduler,
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("init_simulation");

    let handle = meshes.add(Mesh::from(Cuboid::new(1., 1., 1.)));

    scheduler.simulation_data.objects.clear();

    scheduler.simulation_data.objects.push(SimulationObject {
        id: 0,
        name: "cube".to_string(),
        mass: 1.0,
        position: bevy::math::Vec3::new(0.0, 0.0, 0.0),
        velocity: bevy::math::Vec3::new(0.0, 0.0, 0.0),
        acceleration: bevy::math::Vec3::new(0.0, 0.0, 0.0),
        force: bevy::math::Vec3::new(0.0, 0.0, 0.0),
        handle: handle.clone(),
    });

    commands.spawn((
        meshes.add(Cylinder::new(0.2, 1.0)),
        SpatialBundle::INHERITED_IDENTITY,
        InstanceMaterialData(
            (1..=10)
                .flat_map(|x| (1..=10).map(move |y| (x as f32 / 10.0, y as f32 / 10.0)))
                .map(|(x, y)| InstanceData {
                    position: Vec3::new(x * 10.0 - 5.0, y * 10.0 - 5.0, 0.0),
                    scale: 1.0,
                    color: Color::hsla(x * 360., y, 0.5, 1.0).as_rgba_f32(),
                })
                .collect(),
        ),
        NoFrustumCulling,
    ));

    // commands.spawn(PbrBundle {
    //     mesh: handle,
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
    //     transform: Transform::from_xyz(0., 0., 0.),
    //     ..default()
    // });
}

pub fn simulate(task_interface: &mut SimulationTaskInterface) {
    info!("simulate");
}
