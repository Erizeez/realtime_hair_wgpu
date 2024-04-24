use crossbeam_channel::{Receiver, Sender};

use super::data::SimulationTaskInterface;

pub fn init_simulation_channel() -> (
    Sender<SimulationTaskInterface>,
    Receiver<SimulationTaskInterface>,
) {
    crossbeam_channel::unbounded()
}

#[derive(Clone)]
pub struct SimulationResultSender(pub Sender<SimulationTaskInterface>);

#[derive(Clone)]
pub struct SimulationResultReceiver(pub Receiver<SimulationTaskInterface>);
