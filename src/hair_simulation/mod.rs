use bevy::{
    app::{App, Plugin, Update},
    ecs::component::Component,
};

use self::conversion::do_apply;

pub mod conversion;
pub mod data;
pub mod pipeline;
pub mod simulation;

// Marker
#[derive(Component)]
pub struct HairsMarker;

#[derive(Component)]
pub struct HeadMarker;

pub struct HairSimulationPlugin;

impl Plugin for HairSimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, do_apply);
    }
}

const HAIR_THICKNESS: f32 = 0.001;
const HAIR_SEG_LENGTH: f32 = 0.1;
