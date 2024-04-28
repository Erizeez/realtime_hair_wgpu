mod communication;
mod display;
pub mod interfaces;
pub mod scheduler;

use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::Assets,
    ecs::system::{Commands, Query, Res, ResMut},
    input::{keyboard::KeyCode, ButtonInput},
    pbr::StandardMaterial,
    render::mesh::Mesh,
};

use scheduler::*;

use self::display::{setup_display, simulation_text_update_system};

pub struct PhysicSimulationPlugin;

impl Plugin for PhysicSimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_scheduler, setup_display));
        app.add_systems(
            Update,
            (
                schedule_simulation,
                simulation_toggle,
                simulation_text_update_system,
            ),
        );
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
            scheduler.init_scheduler(&mut commands, meshes, materials, true);
        }
    } else if kbd.just_pressed(KeyCode::Escape) {
        let mut scheduler = q.single_mut();
        scheduler.stop_scheduler(&mut commands, meshes);
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
