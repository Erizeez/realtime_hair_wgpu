use bevy::{
    asset::Assets,
    ecs::system::{Commands, ResMut},
    pbr::StandardMaterial,
    render::mesh::Mesh,
};

use super::data::SimulationTaskInterface;

pub trait PhysicalSimulation<T> {
    fn simulate(task_interface: &mut SimulationTaskInterface<T>);
    fn init_simulation(
        commands: &mut Commands,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<StandardMaterial>>,
    ) -> T;
}
