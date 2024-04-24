use crossbeam_channel::{Receiver, Sender};

use super::data::SimulationData;

#[derive(Clone)]
pub struct SimulationDataSender(pub Sender<SimulationData>);

#[derive(Clone)]
pub struct SimulationDataReceiver(pub Receiver<SimulationData>);
