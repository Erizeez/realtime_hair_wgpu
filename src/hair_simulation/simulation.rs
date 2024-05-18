use bevy::{
    log::info,
    math::{Quat, Vec3},
};
extern crate nalgebra as na;

use crate::{
    hair_simulation::{
        data::{convert_to_na_vec3, convert_to_vec3},
        pipeline::der::do_der,
    },
    physic_simulation::interfaces::SimulationTaskInterface,
};

pub fn do_simulate(task_interface: &mut SimulationTaskInterface) {
    info!("simulate");

    let iteration_cnt = task_interface.iteration_cnt;
    let delta = iteration_cnt as f64 / 100.0;
    // task_interface.data.head.position = na::Matrix3x1::<f64>::new(
    //     0.5 * f64::cos(delta) - 0.5,
    //     2.0 + 0.2 * f64::sin(delta * 3.0),
    //     0.5 * f64::sin(delta),
    // );

    // task_interface.data.head.rotation = Quat::from_rotation_x(delta as f32);

    task_interface
        .data
        .hairs
        .strands
        .iter_mut()
        .for_each(|hair| {
            for _ in 0..hair.v_num {
                let attachment: Vec3 =
                    convert_to_vec3(task_interface.data.head.attachments[hair.attachment]);
                hair.v_position[0] =
                    convert_to_na_vec3(task_interface.data.head.rotation * attachment)
                        + task_interface.data.head.position;
                // hair.position[0] = task_interface.data.head.position
                //     + task_interface.data.head.attachments[hair.attachment];
            }
        });

    do_der(task_interface);
}
