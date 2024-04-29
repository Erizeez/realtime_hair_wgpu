use instant::Duration;

use crate::hair_simulation::data::SimulationData;

#[derive(Default, Clone)]
pub struct SimulationTaskInterface {
    pub iteration_cnt: u64,
    pub data: SimulationData,
    pub elapsed: Duration,
}
