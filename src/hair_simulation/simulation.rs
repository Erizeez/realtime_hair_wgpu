use bevy::{
    log::info,
    math::{Quat, Vec3},
};

use crate::{
    hair_simulation::pipeline::der::do_der, physic_simulation::interfaces::SimulationTaskInterface,
};

pub fn do_simulate(task_interface: &mut SimulationTaskInterface) {
    info!("simulate");

    // let iteration_cnt = task_interface.iteration_cnt;
    // let delta = iteration_cnt as f32 / 40.0;
    // task_interface.data.head.position = Vec3::new(
    //     0.5 * f32::cos(delta),
    //     2.0 + 0.2 * f32::sin(delta * 3.0),
    //     0.5 * f32::sin(delta),
    // );

    // task_interface.data.head.rotation = Quat::from_rotation_x(delta);

    // task_interface
    //     .data
    //     .hairs
    //     .strands
    //     .iter_mut()
    //     .for_each(|hair| {
    //         for _ in 0..hair.v_num {
    //             hair.v_position[0] = task_interface.data.head.rotation
    //                 * task_interface.data.head.attachments[hair.attachment]
    //                 + task_interface.data.head.position;
    //             // hair.position[0] = task_interface.data.head.position
    //             //     + task_interface.data.head.attachments[hair.attachment];
    //         }
    //     });

    do_der(task_interface);
}
