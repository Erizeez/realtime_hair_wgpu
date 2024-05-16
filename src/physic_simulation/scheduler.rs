use crate::hair_simulation::conversion::{init_simulation, reset_simulation};
use crate::hair_simulation::data::SimulationData;
use crate::hair_simulation::simulation::do_simulate;

use super::communication::{
    init_simulation_channel, SimulationResultReceiver, SimulationResultSender,
};
use super::interfaces::*;

use bevy::ecs::entity::Entity;
use bevy::tasks::AsyncComputeTaskPool;
use bevy::utils::HashMap;
use bevy::{
    asset::Assets,
    ecs::{
        component::Component,
        system::{Commands, Query, ResMut},
    },
    log::info,
    pbr::StandardMaterial,
    render::mesh::Mesh,
};
use instant::{Duration, Instant};

#[derive(PartialEq, Eq)]
pub enum SimulationStatus {
    Running,
    Paused,
    Stopped,
}

#[derive(Component)]
pub struct PhsicaSimulationScheduler {
    // iteration cnt
    pub iteration_cnt: u64,
    pub last_elapsed: Duration,
    pub status: SimulationStatus,
    pub entities: HashMap<String, Entity>,
    pub simulation_data: SimulationData,
    pub sender: SimulationResultSender,
    pub receiver: SimulationResultReceiver,
    pub is_dirty: bool,
}

impl PhsicaSimulationScheduler {
    pub fn spawn_simulation(&mut self) {
        let thread_pool = AsyncComputeTaskPool::get();
        let data = self.simulation_data.clone();

        let mut task_interface = SimulationTaskInterface {
            iteration_cnt: self.iteration_cnt,
            data,
            delta_time: 1e-3,
            elapsed: Default::default(),
        };

        let sender = self.sender.0.clone();
        thread_pool
            .spawn(async move {
                let start_ts = Instant::now();
                // let start_ts = instant::now();

                do_simulate(&mut task_interface);

                let elapsed = start_ts.elapsed();

                task_interface.elapsed = elapsed;

                sender.send(task_interface).unwrap();
            })
            .detach();
    }

    pub fn init_scheduler(
        &mut self,
        commands: &mut Commands,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<StandardMaterial>>,
        with_start: bool,
    ) {
        init_simulation(self, commands, meshes, materials);
        if with_start {
            self.spawn_simulation();

            self.status = SimulationStatus::Running;
            info!("start_scheduler");
        }
    }
    pub fn resume_scheduler(&mut self) {
        self.spawn_simulation();

        self.status = SimulationStatus::Running;
        info!("resume_scheduler")
    }
    pub fn singlestep_scheduler(&mut self) {
        self.status = SimulationStatus::Paused;
        self.spawn_simulation();

        info!("resume_scheduler")
    }
    pub fn parse_scheduler(&mut self) {
        self.status = SimulationStatus::Paused;
        info!("parse_scheduler");
    }
    pub fn stop_scheduler(&mut self, commands: &mut Commands) {
        self.status = SimulationStatus::Stopped;
        self.iteration_cnt = 0;
        self.last_elapsed = Default::default();

        // Do some cleanup
        reset_simulation(self, commands);

        info!("stop_scheduler");
    }
}

pub fn setup_scheduler(mut commands: Commands) {
    // create our UI root node
    // this is the wrapper/container for the text
    let (sender, receiver) = init_simulation_channel();
    let _ = commands.spawn((PhsicaSimulationScheduler {
        iteration_cnt: 0,
        last_elapsed: Default::default(),
        status: SimulationStatus::Stopped,
        entities: HashMap::new(),
        simulation_data: SimulationData::default(),
        sender: SimulationResultSender(sender),
        receiver: SimulationResultReceiver(receiver),
        is_dirty: false,
    },));
}

pub fn schedule_simulation(mut q: Query<&mut PhsicaSimulationScheduler>) {
    let mut scheduler = q.single_mut();
    let _ = scheduler.receiver.0.try_recv().map(|task_interface| {
        if scheduler.status == SimulationStatus::Stopped {
            return;
        }

        info!("receive data");
        scheduler.simulation_data = task_interface.data;
        info!("elapsed: {:?}", task_interface.elapsed);
        scheduler.iteration_cnt += 1;
        scheduler.last_elapsed = task_interface.elapsed;
        scheduler.is_dirty = true;

        // TODO: update the simulation data to the world

        if scheduler.status == SimulationStatus::Running {
            info!("start next iteration");

            scheduler.spawn_simulation();
        }
    });
}
