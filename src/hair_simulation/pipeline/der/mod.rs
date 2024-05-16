pub mod methods;
pub mod utils;

extern crate nalgebra as na;
use std::f32::consts::PI;

use bevy::log::info;

use crate::{
    hair_simulation::{
        data::Frame,
        pipeline::der::utils::{
            add_to_matrix, calc_nabla_i_kappa_i, calc_nabla_i_kappa_i1, calc_nabla_i_kappa_i_1,
            partial_kappa,
        },
    },
    physic_simulation::interfaces::SimulationTaskInterface,
};

pub fn do_der(task_interface: &mut SimulationTaskInterface) {
    let hairs = &mut task_interface.data.hairs;
    print!("{:?}", hairs.strands.len());

    for strand in hairs.strands.iter_mut() {
        let mut force = na::DMatrix::<f32>::zeros(4 * strand.v_num - 1, 1);
        let mut hessian = na::DMatrix::<f32>::zeros(4 * strand.v_num - 1, 4 * strand.v_num - 1);
        let mut velocity_0 = na::DMatrix::<f32>::zeros(4 * strand.v_num - 1, 1);
        let mut velocity = na::DMatrix::<f32>::zeros(4 * strand.v_num - 1, 1);
        let mut mass = na::DMatrix::<f32>::zeros(4 * strand.v_num - 1, 4 * strand.v_num - 1);

        // Fill mass matrix
        for i in 0..strand.v_num {
            mass[(3 * i, 3 * i)] = strand.v_mass[i];
            mass[(3 * i + 1, 3 * i + 1)] = strand.v_mass[i];
            mass[(3 * i + 2, 3 * i + 2)] = strand.v_mass[i];
        }

        for i in 0..strand.l_num {
            mass[(3 * strand.v_num + i, 3 * strand.v_num + i)] = strand.l_momemtum[i];
        }

        // Fill velocity_0
        for i in 0..strand.v_num {
            velocity_0[(3 * i, 0)] = strand.v_velocity[i].x;
            velocity_0[(3 * i + 1, 0)] = strand.v_velocity[i].y;
            velocity_0[(3 * i + 2, 0)] = strand.v_velocity[i].z;
        }

        for i in 0..strand.l_num {
            velocity_0[(3 * strand.v_num + i, 0)] = strand.l_twist[i];
        }

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
            info!("kappa initialized");
        }

        // Apply stretch
        for i in 1..(strand.v_num as usize - 1) {
            let f_si = PI
                * strand.radius.powf(2.0)
                * strand.youngs
                * ((length_vec[i] / strand.l_initial_length[i] - 1.0)
                    * strand.reference_frame[i].t
                    - (length_vec[i - 1] / strand.l_initial_length[i - 1] - 1.0)
                        * strand.reference_frame[i - 1].t);

            force[i * 3] += f_si[0];
            force[i * 3 + 1] += f_si[1];
            force[i * 3 + 2] += f_si[2];

            let h_i_1 = PI
                * strand.radius.powf(2.0)
                * strand.youngs
                * ((1.0 / strand.l_initial_length[i - 1] - 1.0 / length_vec[i - 1])
                    * na::Matrix3::<f32>::identity()
                    + strand.reference_frame[i - 1].t
                        * strand.reference_frame[i - 1].t.transpose());

            let h_i1 = PI
                * strand.radius.powf(2.0)
                * strand.youngs
                * ((1.0 / strand.l_initial_length[i] - 1.0 / length_vec[i])
                    * na::Matrix3::<f32>::identity()
                    + strand.reference_frame[i].t * strand.reference_frame[i].t.transpose());

            add_to_matrix(
                &mut hessian,
                &h_i_1,
                ((i * 3) as usize, ((i - 1) * 3) as usize),
            );
            add_to_matrix(
                &mut hessian,
                &h_i_1.transpose(),
                (((i - 1) * 3) as usize, (i * 3) as usize),
            );

            add_to_matrix(
                &mut hessian,
                &h_i1,
                ((i * 3) as usize, ((i + 1) * 3) as usize),
            );
            add_to_matrix(
                &mut hessian,
                &h_i1.transpose(),
                (((i + 1) * 3) as usize, (i * 3) as usize),
            );

            add_to_matrix(
                &mut hessian,
                &(-h_i_1 - h_i1),
                ((i * 3) as usize, (i * 3) as usize),
            );
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

        let mut nabla_kappa_vec =
            vec![vec![na::Matrix3x4::<f32>::zeros(); 3]; strand.v_num as usize];

        for i in 1..(strand.v_num as usize - 1) {
            let t_tilde = (strand.reference_frame[i - 1].t + strand.reference_frame[i].t)
                / (1.0
                    + strand.reference_frame[i - 1]
                        .t
                        .dot(&strand.reference_frame[i].t));

            if i - 1 > 1 {
                let nabla_i_kappa_i_1 = calc_nabla_i_kappa_i_1(
                    &e_vec,
                    &t_tilde,
                    &kappa,
                    &strand.reference_frame,
                    &material_frame,
                    i - 1,
                );

                nabla_kappa_vec[i - 1][0] = nabla_i_kappa_i_1;
            }

            {
                let nabla_i_kappa_i = calc_nabla_i_kappa_i(
                    &e_vec,
                    &t_tilde,
                    &kappa,
                    &strand.reference_frame,
                    &material_frame,
                    i,
                );

                nabla_kappa_vec[i][1] = nabla_i_kappa_i;
            }

            if i + 1 < strand.v_num as usize - 1 {
                let nabla_i_kappa_i1 = calc_nabla_i_kappa_i1(
                    &e_vec,
                    &t_tilde,
                    &kappa,
                    &strand.reference_frame,
                    &material_frame,
                    i + 1,
                );

                nabla_kappa_vec[i + 1][2] = nabla_i_kappa_i1;
            }
        }

        // Apply bend
        for i in 1..(strand.v_num as usize - 1) {
            let mut kappa_part = na::Matrix3x1::<f32>::zeros();

            if i - 1 > 1 {
                kappa_part = kappa_part
                    + nabla_kappa_vec[i][0] * (kappa[i - 1] - strand.l_initial_kappa[i - 1])
                        / length_vec[i - 1];
            }

            {
                kappa_part = kappa_part
                    + nabla_kappa_vec[i][1] * (kappa[i] - strand.l_initial_kappa[i])
                        / length_vec[i];
            }

            if i + 1 < strand.v_num as usize - 1 {
                kappa_part = kappa_part
                    + nabla_kappa_vec[i][2] * (kappa[i + 1] - strand.l_initial_kappa[i + 1])
                        / length_vec[i + 1];
            }

            let f_sum = -PI * strand.radius.powf(4.0) * strand.youngs / 8.0 * kappa_part;

            // force[(i * 3) as usize] = force[(i * 3) as usize] + f_sum[0];
            // force[(i * 3 + 1) as usize] = force[(i * 3) as usize] + f_sum[1];
            // force[(i * 3 + 2) as usize] = force[(i * 3) as usize] + f_sum[2];

            // let h_i_i_2 = -PI * strand.radius.powf(4.0) * strand.youngs / 8.0 * ();
        }

        // info!("{:?}", force);

        // Apply twist

        // Apply gravity
        for i in 1..(strand.v_num as usize) {
            force[i * 3] += 0.0;
            force[i * 3 + 1] += -9.8 * strand.v_mass[i];
            force[i * 3 + 2] += 0.0;
        }

        // info!("{:?}", force);
        info!("{:?}", hessian);
        let a = mass.clone() + task_interface.delta_time.powf(2.0) * hessian;

        let b = mass.clone() * velocity_0 + task_interface.delta_time * force;

        // Solve linear system
        velocity = a.lu().solve(&b).unwrap();

        // Update strand states
        for i in 1..(strand.v_num as usize) {
            strand.v_velocity[i] = na::Vector3::new(
                velocity[(i * 3) as usize],
                velocity[(i * 3 + 1) as usize],
                velocity[(i * 3 + 2) as usize],
            );

            strand.v_position[i] =
                strand.v_position[i] + task_interface.delta_time * strand.v_velocity[i];
        }
    }
}
