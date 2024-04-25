use std::marker::PhantomData;

use bevy::{
    app::{App, Plugin},
    asset::Assets,
    ecs::system::{Commands, ResMut},
    pbr::StandardMaterial,
    render::mesh::Mesh,
};

use crate::plugins::physic_simulation::{
    data::SimulationTaskInterface, traits::PhysicalSimulation, PhysicSimulationPlugin,
};

use self::data::SimulationData;

mod data;
mod pipeline;

pub struct RTHairOperations;

impl PhysicalSimulation<SimulationData> for RTHairOperations {
    fn simulate(task_interface: &mut SimulationTaskInterface<SimulationData>) {
        todo!()
    }

    fn init_simulation(
        commands: &mut Commands,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<StandardMaterial>>,
    ) -> SimulationData {
        todo!()
    }
}

pub struct RTHairSimulationPlugin;

impl Plugin for RTHairSimulationPlugin {
    fn build(&self, app: &mut App) {
        let operations = RTHairOperations;
        let plugin = PhysicSimulationPlugin::<SimulationData, RTHairOperations>::new();
        app.add_plugins(plugin);
    }
}
