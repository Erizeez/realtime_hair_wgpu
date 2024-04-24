use std::borrow::Borrow;

use bevy::{
    asset::Assets,
    ecs::system::{Commands, ResMut},
    log::info,
    math::primitives::Cuboid,
    pbr::{PbrBundle, StandardMaterial},
    prelude::default,
    render::{color::Color, mesh::Mesh},
    transform::components::Transform,
};

use super::{
    data::{SimulationData, SimulationObject, SimulationTaskInterface},
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

    commands.spawn(PbrBundle {
        mesh: handle,
        material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
}

pub fn simulate(task_interface: SimulationTaskInterface) {
    info!("simulate");
}
