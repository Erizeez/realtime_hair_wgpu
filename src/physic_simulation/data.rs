use bevy::{asset::Handle, math::Vec3, render::mesh::Mesh};

#[derive(Default, Clone)]
pub struct SimulationData {
    pub objects: Vec<SimulationObject>,
}

#[derive(Clone)]
pub struct SimulationObject {
    pub id: u32,
    pub name: String,
    pub mass: f32,
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub force: Vec3,
    pub handle: Handle<Mesh>,
}
