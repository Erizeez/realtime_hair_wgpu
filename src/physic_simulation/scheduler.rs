use super::communication::SimulationDataReceiver;
use super::communication::SimulationDataSender;
use super::data::*;
use super::simulation::*;

use bevy::tasks::AsyncComputeTaskPool;
use bevy::{
    asset::Assets,
    ecs::{
        component::Component,
        system::{CommandQueue, Commands, Query, ResMut},
    },
    log::info,
    pbr::StandardMaterial,
    render::mesh::Mesh,
    tasks::{block_on, futures_lite::future, Task},
};

#[derive(PartialEq, Eq)]
pub enum SimulationStatus {
    Running,
    Paused,
    Stopped,
}

#[derive(Component)]
pub struct PhsicaSimulationScheduler {
    // iteration cnt
    pub iteration_cnt: u32,
    pub status: SimulationStatus,
    pub simulation_data: SimulationData,
    pub data_sender: SimulationDataSender,
    pub data_receiver: SimulationDataReceiver,
}

impl PhsicaSimulationScheduler {
    pub fn spawn_simulation(&mut self, commands: &mut Commands) {
        let thread_pool = AsyncComputeTaskPool::get();
        let data = self.simulation_data.clone();

        let entity = commands.spawn_empty().id();
        let sender = self.data_sender.0.clone();
        let task = thread_pool
            .spawn(async move {
                let mut commands = CommandQueue::default();
                simulate(data.clone());

                sender.send(data).unwrap();
            })
            .detach();
    }

    pub fn start_scheduler(
        &mut self,
        mut commands: &mut Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        init_simulation(self, commands, meshes, materials);
        self.spawn_simulation(commands);

        self.status = SimulationStatus::Running;
        info!("start_scheduler");
    }
    pub fn resume_scheduler(&mut self) {
        self.status = SimulationStatus::Running;
        info!("resume_scheduler")
    }
    pub fn parse_scheduler(&mut self) {
        self.status = SimulationStatus::Paused;
        info!("parse_scheduler");
    }
    pub fn stop_scheduler(&mut self) {
        self.status = SimulationStatus::Stopped;
        info!("stop_scheduler");
    }
}

#[derive(Component)]
pub struct PhysicSimulationResult(Task<CommandQueue>);

pub fn setup_scheduler(mut commands: Commands) {
    // create our UI root node
    // this is the wrapper/container for the text
    let (sender, receiver) = crossbeam_channel::unbounded::<SimulationData>();
    let _ = commands.spawn((PhsicaSimulationScheduler {
        iteration_cnt: 0,
        status: SimulationStatus::Stopped,
        simulation_data: SimulationData::default(),
        data_sender: SimulationDataSender(sender),
        data_receiver: SimulationDataReceiver(receiver),
    },));
}

pub fn schedule_simulation(
    mut commands: Commands,
    mut q: Query<&mut PhsicaSimulationScheduler>,
    mut simulation_tasks: Query<&mut PhysicSimulationResult>,
) {
    let mut scheduler = q.single_mut();
    scheduler.data_receiver.0.try_recv().map(|data| {
        info!("receive data");
        scheduler.simulation_data = data;
    });
    // let mut simulation_task = simulation_tasks.single_mut();
    // if let Some(mut task_results) = block_on(future::poll_once(&mut simulation_task.0)) {
    //     info!("finish last iteration");
    //     commands.append(&mut task_results);
    //     let mut scheduler = q.single_mut();
    //     scheduler.iteration_cnt += 1;

    //     if scheduler.status == SimulationStatus::Running {
    //         info!("start next iteration");
    //         // TODO: start next iteration
    //         scheduler.spawn_simulation(commands);
    //     }
    // }
}
