mod communication;
mod control;
mod display;
pub mod interfaces;
pub mod scheduler;

use bevy::app::{App, Plugin, Startup, Update};

use scheduler::*;

use self::{
    control::{button_system, keyboard_control, setup_button},
    display::{setup_display, simulation_text_update_system},
};

pub struct PhysicSimulationPlugin;

impl Plugin for PhysicSimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_scheduler, setup_display, setup_button));
        app.add_systems(
            Update,
            (
                schedule_simulation,
                keyboard_control,
                simulation_text_update_system,
                button_system,
            ),
        );
    }
}
