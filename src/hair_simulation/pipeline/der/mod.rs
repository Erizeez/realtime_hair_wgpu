pub mod methods;
pub mod utils;

extern crate nalgebra as na;
use std::f32::consts::PI;

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
            strand.reference_frame[i as usize].b = reference_frame.0;
            strand.reference_frame[i as usize].n = reference_frame.1;
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

        // Update strand states
    }
}
