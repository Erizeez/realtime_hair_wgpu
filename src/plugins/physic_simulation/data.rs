use instant::Duration;

#[derive(Default, Clone)]
pub struct SimulationTaskInterface<T> {
    pub data: T,
    pub elapsed: Duration,
}
