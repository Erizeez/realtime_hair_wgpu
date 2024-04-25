use bevy::{
    asset::Assets,
    ecs::system::{Commands, Query, Res, ResMut},
    input::{keyboard::KeyCode, ButtonInput},
    pbr::StandardMaterial,
    render::mesh::Mesh,
};

use super::{traits::PhysicalSimulation, PhsicaSimulationScheduler, SimulationStatus};

pub fn simulation_toggle<
    T: Clone + Send + Sync + 'static,
    V: PhysicalSimulation<T> + Send + Sync + 'static,
>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut q: Query<&mut PhsicaSimulationScheduler<T, V>>,
    kbd: Res<ButtonInput<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::Space) {
        let mut scheduler = q.single_mut();
        if scheduler.status == SimulationStatus::Running {
            scheduler.parse_scheduler();
        } else if scheduler.status == SimulationStatus::Paused {
            scheduler.resume_scheduler();
        } else if scheduler.status == SimulationStatus::Stopped {
            scheduler.init_scheduler(&mut commands, meshes, materials, true);
        }
    } else if kbd.just_pressed(KeyCode::Escape) {
        let mut scheduler = q.single_mut();
        scheduler.stop_scheduler();
    } else if kbd.just_pressed(KeyCode::KeyN) {
        let mut scheduler = q.single_mut();
        if scheduler.status == SimulationStatus::Running {
            scheduler.parse_scheduler()
        } else if scheduler.status == SimulationStatus::Paused {
            scheduler.singlestep_scheduler();
        } else if scheduler.status == SimulationStatus::Stopped {
            scheduler.init_scheduler(&mut commands, meshes, materials, false);
            scheduler.singlestep_scheduler();
        }
    }
}
