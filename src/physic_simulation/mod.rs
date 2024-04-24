mod communication;
mod data;
mod scheduler;
mod simulation;

use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::Assets,
    ecs::system::{Commands, Query, Res, ResMut},
    input::{keyboard::KeyCode, ButtonInput},
    pbr::StandardMaterial,
    render::mesh::Mesh,
};

use scheduler::*;

pub struct PhysicSimulationPlugin;

impl Plugin for PhysicSimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scheduler);
        app.add_systems(Update, (schedule_simulation, simulation_toggle));
    }
}

fn simulation_toggle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut q: Query<&mut PhsicaSimulationScheduler>,
    kbd: Res<ButtonInput<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::Space) {
        let mut scheduler = q.single_mut();
        if scheduler.status == SimulationStatus::Running {
            scheduler.parse_scheduler();
        } else if scheduler.status == SimulationStatus::Paused {
            scheduler.resume_scheduler();
        } else if scheduler.status == SimulationStatus::Stopped {
            scheduler.start_scheduler(&mut commands, meshes, materials);
        }
    } else if kbd.just_pressed(KeyCode::Escape) {
        let mut scheduler = q.single_mut();
        scheduler.stop_scheduler();
    }
}
