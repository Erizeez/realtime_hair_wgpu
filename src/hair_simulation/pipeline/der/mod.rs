pub mod methods;
pub mod utils;

extern crate nalgebra as na;
use std::f32::consts::PI;

use bevy::log::info;

use crate::{
    hair_simulation::{data::Frame, pipeline::der::utils::partial_kappa},
    physic_simulation::interfaces::SimulationTaskInterface,
};

pub fn do_der(task_interface: &mut SimulationTaskInterface) {
    let hairs = &mut task_interface.data.hairs;
    print!("{:?}", hairs.strands.len());

    for strand in hairs.strands.iter_mut() {
        let mut e_vec = Vec::new();
        let mut length_vec = Vec::new();
        // Update reference frame
        for i in 0..strand.l_num {
            let e = strand.v_position[(i + 1) as usize] - strand.v_position[i as usize];
            let t = e.normalize();
            length_vec.push(e.norm());

            let reference_frame =
                utils::parallel_transport(strand.reference_frame[i as usize].t, t);
            // info!("lastFrame: {:?}", strand.reference_frame[i as usize]);
            // info!("t: {:?}", t);
            // info!("referenceFrame: {:?}", reference_frame);
            strand.reference_frame[i as usize].b = reference_frame.0;
            strand.reference_frame[i as usize].n = reference_frame.2;
            strand.reference_frame[i as usize].t = t;
            e_vec.push(e);
        }

        // Calculate material frame
        let mut material_frame = Vec::new();
        for i in 0..(strand.l_num as usize) {
            let a1 = strand.reference_frame[i as usize].n;
            let a2 = strand.reference_frame[i as usize].b;

            let m1 = a1 * f32::cos(strand.l_twist[i]) + a2 * f32::sin(strand.l_twist[i]);
            let m2 = -a1 * f32::sin(strand.l_twist[i]) + a2 * f32::cos(strand.l_twist[i]);

            material_frame.push(Frame {
                b: m2,
                n: m1,
                t: na::Vector3::zeros(),
            });
        }

        // Calculate kappa_b
        let mut kappa_b = Vec::new();
        for i in 0..(strand.v_num as usize - 1) {
            if i == 0 {
                kappa_b.push(na::Vector3::zeros());
            } else {
                kappa_b.push(
                    (2.0 * strand.reference_frame[i - 1]
                        .t
                        .cross(&strand.reference_frame[i].t))
                        / (1.0
                            + strand.reference_frame[i - 1]
                                .t
                                .dot(&strand.reference_frame[i].t)),
                );
            }
        }

        // Calculate kappa
        let mut kappa = Vec::new();
        for i in 0..(strand.v_num as usize - 1) {
            if i == 0 {
                kappa.push(na::Matrix4x1::new(0.0, 0.0, 0.0, 0.0));
            } else {
                let kappa_i = na::Matrix4x1::new(
                    material_frame[i - 1].b.dot(&kappa_b[i]),
                    material_frame[i].b.dot(&kappa_b[i]),
                    -material_frame[i - 1].n.dot(&kappa_b[i]),
                    -material_frame[i].n.dot(&kappa_b[i]),
                );
                kappa.push(kappa_i)
            }
        }

        if strand.l_initial_kappa.len() == 0 {
            for i in 0..(strand.v_num as usize - 1) {
                strand.l_initial_kappa.push(kappa[i]);
            }
        }

        let mut force = na::DMatrix::<f32>::zeros(4 * strand.v_num - 1, 1);
        // Apply stretch
        for i in 1..(strand.v_num as usize - 1) {
            let f_si = PI
                * strand.radius.powf(2.0)
                * strand.youngs
                * ((length_vec[i] / strand.l_initial_length[i] - 1.0)
                    * strand.reference_frame[i].t
                    - (length_vec[i - 1] / strand.l_initial_length[i - 1] - 1.0)
                        * strand.reference_frame[i - 1].t);

            force[(i * 3) as usize] = force[(i * 3) as usize] + f_si[0];
            force[(i * 3 + 1) as usize] = force[(i * 3 + 1) as usize] + f_si[1];
            force[(i * 3 + 2) as usize] = force[(i * 3 + 2) as usize] + f_si[2];
        }

        let f_s_last = -PI
            * strand.radius.powf(2.0)
            * strand.youngs
            * (length_vec[strand.v_num as usize - 2]
                / strand.l_initial_length[strand.v_num as usize - 2]
                - 1.0)
            * strand.reference_frame[strand.v_num as usize - 2].t;

        force[((strand.v_num - 1) * 3) as usize] =
            force[((strand.v_num - 1) * 3) as usize] + f_s_last[0];
        force[((strand.v_num - 1) * 3 + 1) as usize] =
            force[((strand.v_num - 1) * 3 + 1) as usize] + f_s_last[1];
        force[((strand.v_num - 1) * 3 + 2) as usize] =
            force[((strand.v_num - 1) * 3 + 2) as usize] + f_s_last[2];

        info!("{:?}", force);

        // Apply bend
        for i in 1..(strand.v_num as usize - 1) {
            let t_tilde = (strand.reference_frame[i - 1].t + strand.reference_frame[i].t)
                / (1.0
                    + strand.reference_frame[i - 1]
                        .t
                        .dot(&strand.reference_frame[i].t));

            let mut nabla_i_kappa_i_1 = na::Matrix3x4::<f32>::zeros();
            let mut nabla_i_kappa_i = na::Matrix3x4::<f32>::zeros();
            let mut nabla_i_kappa_i1 = na::Matrix3x4::<f32>::zeros();

            let mut f_sum = na::Matrix3x1::<f32>::zeros();

            if i - 1 > 1 {
                nabla_i_kappa_i_1.set_column(
                    0,
                    &partial_kappa(
                        &e_vec,
                        &t_tilde,
                        &kappa,
                        &strand.reference_frame,
                        &material_frame,
                        i - 1,
                        true,
                        true,
                        false,
                    ),
                );

                nabla_i_kappa_i_1.set_column(
                    1,
                    &partial_kappa(
                        &e_vec,
                        &t_tilde,
                        &kappa,
                        &strand.reference_frame,
                        &material_frame,
                        i - 1,
                        false,
                        true,
                        false,
                    ),
                );

                nabla_i_kappa_i_1.set_column(
                    2,
                    &partial_kappa(
                        &e_vec,
                        &t_tilde,
                        &kappa,
                        &strand.reference_frame,
                        &material_frame,
                        i - 1,
                        true,
                        false,
                        false,
                    ),
                );

                nabla_i_kappa_i_1.set_column(
                    3,
                    &partial_kappa(
                        &e_vec,
                        &t_tilde,
                        &kappa,
                        &strand.reference_frame,
                        &material_frame,
                        i - 1,
                        false,
                        false,
                        false,
                    ),
                );

                f_sum = f_sum
                    + -PI * strand.radius.powf(4.0) * strand.youngs / 8.0 / length_vec[i - 1]
                        * nabla_i_kappa_i_1
                        * (kappa[i - 1] - strand.l_initial_kappa[i - 1]);
            }

            {
                nabla_i_kappa_i.set_column(
                    0,
                    &(partial_kappa(
                        &e_vec,
                        &t_tilde,
                        &kappa,
                        &strand.reference_frame,
                        &material_frame,
                        i,
                        true,
                        true,
                        true,
                    ) - partial_kappa(
                        &e_vec,
                        &t_tilde,
                        &kappa,
                        &strand.reference_frame,
                        &material_frame,
                        i,
                        true,
                        true,
                        false,
                    )),
                );

                nabla_i_kappa_i.set_column(
                    1,
                    &(partial_kappa(
                        &e_vec,
                        &t_tilde,
                        &kappa,
                        &strand.reference_frame,
                        &material_frame,
                        i,
                        false,
                        true,
                        true,
                    ) - partial_kappa(
                        &e_vec,
                        &t_tilde,
                        &kappa,
                        &strand.reference_frame,
                        &material_frame,
                        i,
                        false,
                        true,
                        false,
                    )),
                );

                nabla_i_kappa_i.set_column(
                    2,
                    &(partial_kappa(
                        &e_vec,
                        &t_tilde,
                        &kappa,
                        &strand.reference_frame,
                        &material_frame,
                        i,
                        true,
                        false,
                        true,
                    ) - partial_kappa(
                        &e_vec,
                        &t_tilde,
                        &kappa,
                        &strand.reference_frame,
                        &material_frame,
                        i,
                        true,
                        false,
                        false,
                    )),
                );

                nabla_i_kappa_i.set_column(
                    3,
                    &(partial_kappa(
                        &e_vec,
                        &t_tilde,
                        &kappa,
                        &strand.reference_frame,
                        &material_frame,
                        i,
                        false,
                        false,
                        true,
                    ) - partial_kappa(
                        &e_vec,
                        &t_tilde,
                        &kappa,
                        &strand.reference_frame,
                        &material_frame,
                        i,
                        false,
                        false,
                        false,
                    )),
                );

                f_sum = f_sum
                    + -PI * strand.radius.powf(4.0) * strand.youngs / 8.0 / length_vec[i]
                        * nabla_i_kappa_i
                        * (kappa[i] - strand.l_initial_kappa[i]);
            }

            if i + 1 < strand.v_num as usize - 1 {
                nabla_i_kappa_i1.set_column(
                    0,
                    &-partial_kappa(
                        &e_vec,
                        &t_tilde,
                        &kappa,
                        &strand.reference_frame,
                        &material_frame,
                        i + 1,
                        true,
                        true,
                        true,
                    ),
                );

                nabla_i_kappa_i1.set_column(
                    1,
                    &-partial_kappa(
                        &e_vec,
                        &t_tilde,
                        &kappa,
                        &strand.reference_frame,
                        &material_frame,
                        i + 1,
                        false,
                        true,
                        true,
                    ),
                );

                nabla_i_kappa_i1.set_column(
                    2,
                    &-partial_kappa(
                        &e_vec,
                        &t_tilde,
                        &kappa,
                        &strand.reference_frame,
                        &material_frame,
                        i + 1,
                        true,
                        false,
                        true,
                    ),
                );

                nabla_i_kappa_i1.set_column(
                    3,
                    &-partial_kappa(
                        &e_vec,
                        &t_tilde,
                        &kappa,
                        &strand.reference_frame,
                        &material_frame,
                        i + 1,
                        false,
                        false,
                        true,
                    ),
                );

                f_sum = f_sum
                    + -PI * strand.radius.powf(4.0) * strand.youngs / 8.0 / length_vec[i + 1]
                        * nabla_i_kappa_i1
                        * (kappa[i + 1] - strand.l_initial_kappa[i + 1]);

                force[(i * 3) as usize] = f_sum[0];
                force[(i * 3 + 1) as usize] = f_sum[1];
                force[(i * 3 + 2) as usize] = f_sum[2];
            }
        }

        // Apply twist

        // info!("{:?}", force);
        // Apply gravity
        for i in 0..(strand.v_num as usize) {
            force[(i * 3) as usize] = force[(i * 3) as usize] + 0.0;
            force[(i * 3 + 1) as usize] = force[(i * 3 + 1) as usize] + -9.8;
            force[(i * 3 + 2) as usize] = force[(i * 3 + 2) as usize] + 0.0;
        }

        // Update strand states
        for i in 1..(strand.v_num as usize) {
            let mut v = na::Vector3::zeros();
            v[0] = strand.v_velocity[i].x;
            v[1] = strand.v_velocity[i].y;
            v[2] = strand.v_velocity[i].z;

            let mut f = na::Vector3::zeros();
            f[0] = force[(i * 3) as usize];
            f[1] = force[(i * 3 + 1) as usize];
            f[2] = force[(i * 3 + 2) as usize];

            let mut p = na::Vector3::zeros();
            p[0] = strand.v_position[i].x;
            p[1] = strand.v_position[i].y;
            p[2] = strand.v_position[i].z;

            let mut a = f / strand.v_mass[i];

            let mut v_new = v + a * task_interface.delta_time;
            let mut p_new = p + v_new * task_interface.delta_time;

            strand.v_velocity[i].x = v_new[0];
            strand.v_velocity[i].y = v_new[1];
            strand.v_velocity[i].z = v_new[2];

            strand.v_position[i].x = p_new[0];
            strand.v_position[i].y = p_new[1];
            strand.v_position[i].z = p_new[2];
        }
    }
}
