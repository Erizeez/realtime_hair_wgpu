mod communication;
mod control;
pub mod data;
mod display;
pub mod scheduler;
pub mod simulation;
pub mod traits;

use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::bundle::Bundle,
};
use std::marker::PhantomData;
use traits::PhysicalSimulation;

use scheduler::*;

use self::{
    control::simulation_toggle,
    display::{setup_display, simulation_text_update_system},
};

pub struct PhysicSimulationPlugin<T, V: PhysicalSimulation<T>>(
    pub PhantomData<V>,
    pub PhantomData<T>,
);

impl<T, V: PhysicalSimulation<T>> PhysicSimulationPlugin<T, V> {
    pub fn new() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<
        T: Default + Clone + Send + Sync + 'static,
        V: PhysicalSimulation<T> + Default + Clone + Send + Sync + Bundle + 'static,
    > Plugin for PhysicSimulationPlugin<T, V>
{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_scheduler::<T, V>, setup_display));
        app.add_systems(
            Update,
            (
                schedule_simulation::<T, V>,
                simulation_toggle::<T, V>,
                simulation_text_update_system::<T, V>,
            ),
        );
    }
}
