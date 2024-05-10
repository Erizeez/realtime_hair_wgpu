use std::f32::consts::PI;

use bevy::{
    asset::Assets,
    ecs::system::{Commands, Query, ResMut},
    log::info,
    math::{
        primitives::{Cylinder, Sphere},
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
    physic_simulation::scheduler::PhsicaSimulationScheduler,
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

    scheduler.simulation_data.hairs.strands.clear();
    scheduler.entities.clear();
}

pub fn init_simulation(
    scheduler: &mut PhsicaSimulationScheduler,
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("init_simulation");

    let head_position = Vec3::new(0., 2., 0.);
    let head_radius = 0.1;

    scheduler.simulation_data =
        generate_batch_hair_strands(head_position.clone(), head_radius, PI / 4.0, 10, 0.5, 10);

    let hair_data: Vec<InstanceData> = scheduler
        .simulation_data
        .hairs
        .strands
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
                mesh: meshes.add(Sphere::new(head_radius)),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(1.0, 0.8, 0.6),
                    ..Default::default()
                }),
                transform: Transform::from_translation(head_position),
                ..Default::default()
            })
            .insert(HeadMarker)
            .id(),
    );
}

pub fn do_apply(
    mut scheduler_query: Query<&mut PhsicaSimulationScheduler>,
    mut hairs_query: Query<(&HairsMarker, &mut InstanceMaterialData)>,
    mut head_query: Query<(&HeadMarker, &mut Transform)>,
) {
    match scheduler_query.get_single_mut() {
        Ok(mut scheduler) => {
            if scheduler.is_dirty {
                scheduler.is_dirty = false;

                match head_query.get_single_mut() {
                    Ok((_, mut head_transform)) => {
                        head_transform.translation = scheduler.simulation_data.head.position;
                        head_transform.rotation = scheduler.simulation_data.head.rotation;
                    }
                    Err(_) => {
                        info!("head not found");
                    }
                }

                match hairs_query.get_single_mut() {
                    Ok((_, mut instance_material_data)) => {
                        let hair_data: Vec<InstanceData> = scheduler
                            .simulation_data
                            .hairs
                            .strands
                            .iter()
                            .flat_map(|hair| hair.to_instance_data())
                            .collect();

                        instance_material_data.0 = hair_data;
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
