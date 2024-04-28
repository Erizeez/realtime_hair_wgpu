use std::f32::consts::PI;

use bevy::{
    asset::Assets,
    ecs::system::{Commands, Query, ResMut},
    log::info,
    math::{
        primitives::{Cuboid, Cylinder, Sphere},
        Vec3,
    },
    pbr::{PbrBundle, StandardMaterial},
    prelude::SpatialBundle,
    render::{color::Color, mesh::Mesh, view::NoFrustumCulling},
    transform::components::Transform,
};

use crate::{
    hair_simulation::{
        data::generate_batch_hair_strands, HeadMarker, HAIR_SEG_LENGTH, HAIR_THICKNESS,
    },
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

    scheduler.simulation_data.hairs.clear();
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

    scheduler.simulation_data.hairs =
        generate_batch_hair_strands(Vec3::new(0., 2., 0.), 0.1, PI / 4.0, 2, 0.5, 10);

    let hair_data: Vec<InstanceData> = scheduler
        .simulation_data
        .hairs
        .iter()
        .flat_map(|hair| hair.to_instance_data())
        .collect();

    scheduler.entities.insert(
        "hairs".to_string(),
        commands
            .spawn((
                meshes.add(Cylinder::new(HAIR_THICKNESS, HAIR_SEG_LENGTH)),
                HairsMarker,
                SpatialBundle::INHERITED_IDENTITY,
                InstanceMaterialData(hair_data),
                NoFrustumCulling,
            ))
            .id(),
    );

    scheduler.entities.insert(
        "head".to_string(),
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Sphere::new(0.1)),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.8, 0.7, 0.6),
                    ..Default::default()
                }),
                transform: Transform::from_xyz(0., 2., 0.),
                ..Default::default()
            })
            .insert(HeadMarker)
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
    // info!("apply");

    match scheduler_query.get_single_mut() {
        Ok(mut scheduler) => {
            if scheduler.is_dirty {
                scheduler.is_dirty = false;

                match hairs_query.get_single_mut() {
                    Ok((_, mut instance_material_data)) => {
                        info!("hairs moved");
                        for instance_data in instance_material_data.0.iter_mut() {
                            // instance_data.position.x += 0.01;
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
