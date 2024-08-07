pub mod methods;
pub mod utils;

extern crate nalgebra as na;
use core::task;
use std::f64::consts::PI;

use bevy::{log::info, scene::ron::de};

use crate::{
    hair_simulation::{
        data::{self, Frame},
        pipeline::der::utils::{
            add_to_matrix, calc_nabla_i_kappa_i, calc_nabla_i_kappa_i1, calc_nabla_i_kappa_i_1,
        },
    },
    physic_simulation::interfaces::SimulationTaskInterface,
};

pub const MAX_T_DOT: f64 = 100.0;

pub fn do_der(task_interface: &mut SimulationTaskInterface) {
    let hairs = &mut task_interface.data.hairs;
    let head = &task_interface.data.head;
    print!("{:?}", hairs.strands.len());

    for strand in hairs.strands.iter_mut() {
        let mut force = na::DMatrix::<f64>::zeros(4 * strand.v_num - 1, 1);
        let mut hessian = na::DMatrix::<f64>::zeros(4 * strand.v_num - 1, 4 * strand.v_num - 1);
        let mut velocity_0 = na::DMatrix::<f64>::zeros(4 * strand.v_num - 1, 1);
        let mut velocity = na::DMatrix::<f64>::zeros(4 * strand.v_num - 1, 1);
        let mut mass = na::DMatrix::<f64>::zeros(4 * strand.v_num - 1, 4 * strand.v_num - 1);

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
            velocity_0[(3 * strand.v_num + i, 0)] = strand.l_angular[i];
        }

        let mut e_vec = Vec::new();
        let mut length_vec = Vec::new();
        // Update reference frame
        for i in 0..strand.l_num {
            let e = strand.v_position[i + 1] - strand.v_position[i];
            let t = e.normalize();
            // info!("{:?}", t);
            length_vec.push(e.norm());

            let reference_frame = utils::parallel_transport(strand.reference_frame[i].t, t.clone());

            strand.reference_frame[i].b = reference_frame.0;
            strand.reference_frame[i].n = reference_frame.2;
            strand.reference_frame[i].t = t;
            e_vec.push(e);
        }

        // Calculate material frame
        let mut material_frame = Vec::new();
        for i in 0..(strand.l_num) {
            let a1 = strand.reference_frame[i].n;
            let a2 = strand.reference_frame[i].b;

            let m1 = a1 * f64::cos(strand.l_twist[i]) + a2 * f64::sin(strand.l_twist[i]);
            let m2 = -a1 * f64::sin(strand.l_twist[i]) + a2 * f64::cos(strand.l_twist[i]);

            material_frame.push(Frame {
                b: m2,
                n: m1,
                t: na::Vector3::zeros(),
            });
        }

        // Calculate kappa_b
        let mut kappa_b = Vec::new();
        for i in 0..(strand.v_num - 1) {
            if i == 0 {
                kappa_b.push(na::Vector3::zeros());
            } else {
                let mut t_dot = strand.reference_frame[i - 1]
                    .t
                    .dot(&strand.reference_frame[i].t);
                if t_dot > MAX_T_DOT {
                    t_dot = MAX_T_DOT;
                } else if t_dot < -MAX_T_DOT {
                    t_dot = -MAX_T_DOT;
                }
                let temp_kappa_b = (2.0
                    * strand.reference_frame[i - 1]
                        .t
                        .cross(&strand.reference_frame[i].t))
                    / (1.0 + t_dot);
                if temp_kappa_b.norm() > 1.0 {
                    kappa_b.push(temp_kappa_b.normalize() * 1.0);
                } else {
                    kappa_b.push(temp_kappa_b);
                }
            }
        }

        // Calculate kappa
        let mut kappa = Vec::new();
        for i in 0..(strand.v_num - 1) {
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
            for i in 0..(strand.v_num - 1) {
                if i == 0 {
                    strand
                        .l_initial_kappa
                        .push(na::Matrix4x1::new(0.0, 0.0, 0.0, 0.0));
                } else {
                    strand.l_initial_kappa.push(kappa[i]);
                }
            }
            info!("kappa initialized");
        }

        // Apply stretch
        for i in 0..strand.l_num {
            let f_si = PI
                * strand.radius.powi(2)
                * strand.youngs
                * ((length_vec[i] / strand.l_rest_length[i] - 1.0) * strand.reference_frame[i].t);

            if i > strand.last_pin {
                force[i * 3] += f_si[0];
                force[i * 3 + 1] += f_si[1];
                force[i * 3 + 2] += f_si[2];
            }

            force[(i + 1) * 3] -= f_si[0];
            force[(i + 1) * 3 + 1] -= f_si[1];
            force[(i + 1) * 3 + 2] -= f_si[2];

            let h_i = PI
                * strand.radius.powi(2)
                * strand.youngs
                * (1.0 / strand.l_rest_length[i]
                    * strand.reference_frame[i].t
                    * strand.reference_frame[i].t.transpose());
            // info!("{:?}", h_i);

            if i > strand.last_pin {
                add_to_matrix(&mut hessian, &h_i, ((i * 3), (i * 3)));
                add_to_matrix(&mut hessian, &-h_i, ((i * 3), ((i + 1) * 3)));
                add_to_matrix(&mut hessian, &-h_i, (((i + 1) * 3), (i * 3)));
            }

            add_to_matrix(&mut hessian, &h_i, (((i + 1) * 3), ((i + 1) * 3)));
        }
        // info!("{:?}", force);

        let mut nabla_kappa_vec = vec![vec![na::Matrix4x3::<f64>::zeros(); 3]; strand.v_num];

        for i in 1..(strand.v_num - 1) {
            let t_tilde = (strand.reference_frame[i - 1].t + strand.reference_frame[i].t)
                / (1.0
                    + strand.reference_frame[i - 1]
                        .t
                        .dot(&strand.reference_frame[i].t));

            if i - 1 > 0 {
                let nabla_i_kappa_i_1 = calc_nabla_i_kappa_i_1(
                    &e_vec,
                    &t_tilde,
                    &kappa,
                    &strand.reference_frame,
                    &material_frame,
                    i - 1,
                );
                // info!("nabla_i_kappa_i_1{:?}", nabla_i_kappa_i_1);
                nabla_kappa_vec[i][0] = nabla_i_kappa_i_1;
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
                // info!("nabla_i_kappa_i{:?}", nabla_i_kappa_i);
                nabla_kappa_vec[i][1] = nabla_i_kappa_i;
            }

            if i + 1 < strand.v_num - 1 {
                let nabla_i_kappa_i1 = calc_nabla_i_kappa_i1(
                    &e_vec,
                    &t_tilde,
                    &kappa,
                    &strand.reference_frame,
                    &material_frame,
                    i + 1,
                );
                // info!("nabla_i_kappa_i1{:?}", nabla_i_kappa_i1);
                nabla_kappa_vec[i][2] = nabla_i_kappa_i1;
            }
        }

        // Apply bend
        for i in 1..(strand.v_num - 1) {
            // Calc bend force
            let mut kappa_part = na::Matrix3x1::<f64>::zeros();

            if i - 1 > 0 {
                kappa_part = kappa_part
                    + nabla_kappa_vec[i][0].transpose()
                        * (kappa[i - 1] - strand.l_initial_kappa[i - 1])
                        / length_vec[i - 1];
            }

            {
                kappa_part = kappa_part
                    + nabla_kappa_vec[i][1].transpose() * (kappa[i] - strand.l_initial_kappa[i])
                        / length_vec[i];
            }

            if i + 1 < strand.v_num - 1 {
                kappa_part = kappa_part
                    + nabla_kappa_vec[i][2].transpose()
                        * (kappa[i + 1] - strand.l_initial_kappa[i + 1])
                        / length_vec[i + 1];
            }

            let h_factor = PI * strand.radius.powf(4.0) * strand.youngs / 8.0;

            let f_sum = -h_factor * kappa_part;
            // info!("{:?}", f_sum);

            force[i * 3] += f_sum[0];
            force[i * 3 + 1] += f_sum[1];
            force[i * 3 + 2] += f_sum[2];

            // Calc bend force torque

            // Calc bend force hessian

            let mut h_i_i =
                nabla_kappa_vec[i][1].transpose() * nabla_kappa_vec[i][1] / length_vec[i];

            if i >= 2 {
                let mut h_i_i_2 = nabla_kappa_vec[i][0].transpose() * nabla_kappa_vec[i - 2][2]
                    / length_vec[i - 1];

                h_i_i_2 *= h_factor;
                add_to_matrix(&mut hessian, &h_i_i_2, ((i * 3), ((i - 2) * 3)));
                // add_to_matrix(&mut hessian, &h_i_i_2.transpose(), (((i - 2) * 3), (i * 3)));
            }

            if i >= 1 {
                let mut h_i_i_1 = nabla_kappa_vec[i][0].transpose() * nabla_kappa_vec[i - 1][1]
                    / length_vec[i - 1]
                    + nabla_kappa_vec[i][1].transpose() * nabla_kappa_vec[i - 1][2] / length_vec[i];

                h_i_i_1 *= h_factor;
                add_to_matrix(&mut hessian, &h_i_i_1, ((i * 3), ((i - 1) * 3)));
                // add_to_matrix(&mut hessian, &h_i_i_1.transpose(), (((i - 1) * 3), (i * 3)));

                h_i_i +=
                    nabla_kappa_vec[i][0].transpose() * nabla_kappa_vec[i][0] / length_vec[i - 1];
            }

            if i + 1 < strand.l_num {
                let mut h_i_i1 = nabla_kappa_vec[i][1].transpose() * nabla_kappa_vec[i + 1][0]
                    / length_vec[i]
                    + nabla_kappa_vec[i][2].transpose() * nabla_kappa_vec[i + 1][1]
                        / length_vec[i + 1];

                h_i_i1 *= h_factor;
                add_to_matrix(&mut hessian, &h_i_i1, ((i * 3), ((i + 1) * 3)));
                // add_to_matrix(&mut hessian, &h_i_i1.transpose(), (((i + 1) * 3), (i * 3)));

                h_i_i +=
                    nabla_kappa_vec[i][2].transpose() * nabla_kappa_vec[i][2] / length_vec[i + 1];
            }

            if i + 2 < strand.l_num {
                let mut h_i_i2 = nabla_kappa_vec[i][2].transpose() * nabla_kappa_vec[i + 2][0]
                    / length_vec[i + 1];

                h_i_i2 *= h_factor;
                add_to_matrix(&mut hessian, &h_i_i2, ((i * 3), ((i + 2) * 3)));
                // add_to_matrix(&mut hessian, &h_i_i2.transpose(), (((i + 2) * 3), (i * 3)));
            }

            h_i_i *= h_factor;
            add_to_matrix(&mut hessian, &h_i_i, ((i * 3), (i * 3)));
        }

        // for i in 0..strand.l_num {
        //     let mut nabla_i_kappa_i = na::Matrix4x1::<f64>::zeros();

        //     nabla_i_kappa_i[1] = -material_frame[i].n.dot(&kappa_b[i]);
        //     nabla_i_kappa_i[3] = -material_frame[i].b.dot(&kappa_b[i]);
        //     let mut tah = nabla_i_kappa_i.transpose() * (kappa[i] - strand.l_initial_kappa[i])
        //         / length_vec[i];

        //     if i + 1 < strand.l_num {
        //         let mut nabla_i_kappa_i1 = na::Matrix4x1::<f64>::zeros();
        //         nabla_i_kappa_i1[0] = -material_frame[i].n.dot(&kappa_b[i + 1]);
        //         nabla_i_kappa_i1[2] = -material_frame[i].b.dot(&kappa_b[i + 1]);

        //         tah += nabla_i_kappa_i1.transpose()
        //             * (kappa[i + 1] - strand.l_initial_kappa[i + 1])
        //             / length_vec[i + 1];
        //     }

        //     tah *= -PI * strand.radius.powf(4.0) * strand.youngs / 8.0;

        //     force[strand.v_num * 3 + i] += tah[0];
        // }

        // info!("{:?}", force);

        // Apply twist

        // Apply gravity
        for i in (strand.last_pin + 1)..(strand.v_num) {
            force[i * 3] += 0.0;
            force[i * 3 + 1] -= 9.8 * strand.v_mass[i];
            force[i * 3 + 2] += 0.0;

            // hessian[(i * 3 + 1, i * 3 + 1)] += 9.8;
        }

        // Apply force from head
        for i in (strand.last_pin + 1)..(strand.v_num) {
            let velocity_norm = strand.v_velocity[i].norm();
            let distance = (strand.v_position[i] - head.position).norm();
            let direction = (strand.v_position[i] - head.position).normalize();
            let depth = distance - head.radius - 0.01;
            if depth < 0.0 {
                let force_head = direction * depth * depth * 20.0 * velocity_norm;
                force[i * 3] += force_head.x;
                force[i * 3 + 1] += force_head.y;
                force[i * 3 + 2] += force_head.z;
            }
        }

        // info!("{:?}", force);
        // info!("{:?}", hessian);
        let a = mass.clone() + task_interface.delta_time.powi(2) * hessian;

        let b = mass.clone() * velocity_0 + task_interface.delta_time * force;

        // Solve linear system
        velocity = a.lu().solve(&b).unwrap();

        // Update strand states
        for i in (strand.last_pin + 1)..(strand.v_num) {
            strand.v_velocity[i] =
                na::Vector3::new(velocity[i * 3], velocity[i * 3 + 1], velocity[i * 3 + 2]);

            strand.v_position[i] =
                strand.v_position[i] + task_interface.delta_time * strand.v_velocity[i];
        }

        // for i in 0..(strand.l_num) {
        //     strand.l_angular[i] = velocity[3 * strand.v_num + i];

        //     strand.l_twist[i] += task_interface.delta_time * strand.l_angular[i];
        // }
    }
}
