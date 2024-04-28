use bevy::{
    asset::Assets,
    ecs::system::{Commands, Query, ResMut},
    hierarchy::BuildChildren,
    log::info,
    math::{
        primitives::{Cuboid, Cylinder},
        Vec3,
    },
    pbr::StandardMaterial,
    prelude::SpatialBundle,
    render::{color::Color, mesh::Mesh, view::NoFrustumCulling},
};

use crate::{
    hair_simulation::data::SimulationObject,
    physic_simulation::{
        interfaces::SimulationTaskInterface, scheduler::PhsicaSimulationScheduler,
    },
    plugins::instanced_mesh::{InstanceData, InstanceMaterialData},
};

use super::HairsMarker;

pub fn reset_simulation(
    scheduler: &mut PhsicaSimulationScheduler,
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (_, entity) in scheduler.entities.iter() {
        commands.entity(*entity).despawn();
    }

    scheduler.simulation_data.objects.clear();
    scheduler.entities.clear();
}

pub fn init_simulation(
    scheduler: &mut PhsicaSimulationScheduler,
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("init_simulation");

    let handle = meshes.add(Mesh::from(Cuboid::new(1., 1., 1.)));

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

    scheduler.entities.insert(
        "hairs".to_string(),
        commands
            .spawn((
                meshes.add(Cylinder::new(0.2, 1.0)),
                HairsMarker,
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
            ))
            .id(),
    );

    // commands.spawn(PbrBundle {
    //     mesh: handle,
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
    //     transform: Transform::from_xyz(0., 0., 0.),
    //     ..default()
    // });
}

pub fn do_simulate(task_interface: &mut SimulationTaskInterface) {
    info!("simulate");
}

pub fn do_apply(
    mut scheduler_query: Query<&mut PhsicaSimulationScheduler>,
    mut hairs_query: Query<(&HairsMarker, &mut InstanceMaterialData)>,
) {
    info!("apply");

    match scheduler_query.get_single_mut() {
        Ok(mut scheduler) => {
            if scheduler.is_dirty {
                scheduler.is_dirty = false;

                match hairs_query.get_single_mut() {
                    Ok((_, mut instance_material_data)) => {
                        info!("hairs moved");
                        for instance_data in instance_material_data.0.iter_mut() {
                            instance_data.position.x += 0.01;
                        }
                    }
                    Err(_) => {
                        info!("hairs not found");
                    }
                }
            }
        }
        Err(_) => {
            info!("scheduler not found");
        }
    }
}
