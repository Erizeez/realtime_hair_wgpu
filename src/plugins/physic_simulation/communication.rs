use crossbeam_channel::{Receiver, Sender};

use super::data::SimulationTaskInterface;

pub fn init_simulation_channel<T>() -> (
    Sender<SimulationTaskInterface<T>>,
    Receiver<SimulationTaskInterface<T>>,
) {
    crossbeam_channel::unbounded()
}

#[derive(Clone)]
pub struct SimulationResultSender<T>(pub Sender<SimulationTaskInterface<T>>);

#[derive(Clone)]
pub struct SimulationResultReceiver<T>(pub Receiver<SimulationTaskInterface<T>>);
