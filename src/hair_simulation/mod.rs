use bevy::{
    app::{App, Plugin, Update},
    ecs::component::Component,
};

use self::simulation::do_apply;

pub mod data;
pub mod pipeline;
pub mod simulation;

// Marker
#[derive(Component)]
pub struct HairsMarker;

pub struct HairSimulationPlugin;

impl Plugin for HairSimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, do_apply);
    }
}
