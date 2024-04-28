use instant::Duration;

use crate::hair_simulation::data::SimulationData;

#[derive(Default, Clone)]
pub struct SimulationTaskInterface {
    pub data: SimulationData,
    pub elapsed: Duration,
}
