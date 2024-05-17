use bevy::{log::info, utils::info};

use crate::hair_simulation::data::Frame;
extern crate nalgebra as na;

pub fn parallel_transport(
    t0: na::Vector3<f64>,
    t1: na::Vector3<f64>,
) -> (na::Vector3<f64>, na::Vector3<f64>, na::Vector3<f64>) {
    let mut b = t0.cross(&t1);
    if b.norm() == 0.0 {
        b = na::Vector3::new(1.0, 0.0, 0.0);
    }
    let n0: na::Matrix<f64, na::Const<3>, na::Const<1>, na::ArrayStorage<f64, 3, 1>> = t0.cross(&b);
    let n1 = t1.cross(&b);

    // info!("n0: {:?}", n0);
    // info!("n1: {:?}", n1);
    // info!("b: {:?}", b);

    (b.normalize(), n0.normalize(), n1.normalize())
}

pub fn partial_kappa(
    e_vec: &Vec<na::Vector3<f64>>,
    t_tilde: &na::Vector3<f64>,
    kappa: &Vec<na::Matrix4x1<f64>>,
    reference_frame: &Vec<Frame>,
    material_frame: &Vec<Frame>,
    // Equals to i
    index: usize,
    // true means i-1, false means i
    kappa_up_index_b: bool,
    // true means 1, false means 2
    kappa_sub_index_b: bool,
    // true means i-1, false means i
    e_index_b: bool,
) -> na::Matrix3x1<f64> {
    let kappa_up_index;

    // info!("index: {:?}", index);
    // info!("kappa: {:?}", kappa);

    let e_index;
    let t_index;
    let mut kappa_index = 0;
    let mut latter_sign = 1;

    if kappa_up_index_b {
        kappa_up_index = index - 1;
    } else {
        kappa_up_index = index;
        kappa_index = kappa_index + 2;
    }

    let m;
    if kappa_sub_index_b {
        m = material_frame[kappa_up_index].b;
    } else {
        kappa_index = kappa_index + 1;
        latter_sign = latter_sign * -1;
        m = material_frame[kappa_up_index].n;
    }

    if e_index_b {
        e_index = index - 1;
        t_index = index;
    } else {
        e_index = index;
        t_index = index - 1;
        latter_sign = latter_sign * -1;
    }

    let e = e_vec[e_index];
    let kappa_part = -kappa[index][kappa_index] * t_tilde;
    let latter_part = (2.0 * reference_frame[t_index].t.cross(&m))
        / (1.0 + reference_frame[index - 1].t.dot(&reference_frame[index].t));

    let result = 1.0 / e.norm() * (kappa_part + latter_sign as f64 * latter_part);
    // info!("result: {:?}", result);
    result
}

pub fn calc_nabla_i_kappa_i_1(
    e_vec: &Vec<na::Vector3<f64>>,
    t_tilde: &na::Vector3<f64>,
    kappa: &Vec<na::Matrix4x1<f64>>,
    reference_frame: &Vec<Frame>,
    material_frame: &Vec<Frame>,
    index: usize,
) -> na::Matrix4x3<f64> {
    na::Matrix4x3::<f64>::from_rows(&[
        partial_kappa(
            &e_vec,
            &t_tilde,
            &kappa,
            &reference_frame,
            &material_frame,
            index,
            true,
            true,
            false,
        )
        .transpose(),
        partial_kappa(
            &e_vec,
            &t_tilde,
            &kappa,
            &reference_frame,
            &material_frame,
            index,
            false,
            true,
            false,
        )
        .transpose(),
        partial_kappa(
            &e_vec,
            &t_tilde,
            &kappa,
            &reference_frame,
            &material_frame,
            index,
            true,
            false,
            false,
        )
        .transpose(),
        partial_kappa(
            &e_vec,
            &t_tilde,
            &kappa,
            &reference_frame,
            &material_frame,
            index,
            false,
            false,
            false,
        )
        .transpose(),
    ])
}

pub fn calc_nabla_i_kappa_i(
    e_vec: &Vec<na::Vector3<f64>>,
    t_tilde: &na::Vector3<f64>,
    kappa: &Vec<na::Matrix4x1<f64>>,
    reference_frame: &Vec<Frame>,
    material_frame: &Vec<Frame>,
    index: usize,
) -> na::Matrix4x3<f64> {
    na::Matrix4x3::<f64>::from_rows(&[
        (partial_kappa(
            &e_vec,
            &t_tilde,
            &kappa,
            &reference_frame,
            &material_frame,
            index,
            true,
            true,
            true,
        ) - partial_kappa(
            &e_vec,
            &t_tilde,
            &kappa,
            &reference_frame,
            &material_frame,
            index,
            true,
            true,
            false,
        ))
        .transpose(),
        (partial_kappa(
            &e_vec,
            &t_tilde,
            &kappa,
            &reference_frame,
            &material_frame,
            index,
            false,
            true,
            true,
        ) - partial_kappa(
            &e_vec,
            &t_tilde,
            &kappa,
            &reference_frame,
            &material_frame,
            index,
            false,
            true,
            false,
        ))
        .transpose(),
        (partial_kappa(
            &e_vec,
            &t_tilde,
            &kappa,
            &reference_frame,
            &material_frame,
            index,
            true,
            false,
            true,
        ) - partial_kappa(
            &e_vec,
            &t_tilde,
            &kappa,
            &reference_frame,
            &material_frame,
            index,
            true,
            false,
            false,
        ))
        .transpose(),
        (partial_kappa(
            &e_vec,
            &t_tilde,
            &kappa,
            &reference_frame,
            &material_frame,
            index,
            false,
            false,
            true,
        ) - partial_kappa(
            &e_vec,
            &t_tilde,
            &kappa,
            &reference_frame,
            &material_frame,
            index,
            false,
            false,
            false,
        ))
        .transpose(),
    ])
}

pub fn calc_nabla_i_kappa_i1(
    e_vec: &Vec<na::Vector3<f64>>,
    t_tilde: &na::Vector3<f64>,
    kappa: &Vec<na::Matrix4x1<f64>>,
    reference_frame: &Vec<Frame>,
    material_frame: &Vec<Frame>,
    index: usize,
) -> na::Matrix4x3<f64> {
    na::Matrix4x3::<f64>::from_rows(&[
        -partial_kappa(
            &e_vec,
            &t_tilde,
            &kappa,
            &reference_frame,
            &material_frame,
            index,
            true,
            true,
            true,
        )
        .transpose(),
        -partial_kappa(
            &e_vec,
            &t_tilde,
            &kappa,
            &reference_frame,
            &material_frame,
            index,
            false,
            true,
            true,
        )
        .transpose(),
        -partial_kappa(
            &e_vec,
            &t_tilde,
            &kappa,
            &reference_frame,
            &material_frame,
            index,
            true,
            false,
            true,
        )
        .transpose(),
        -partial_kappa(
            &e_vec,
            &t_tilde,
            &kappa,
            &reference_frame,
            &material_frame,
            index,
            false,
            false,
            true,
        )
        .transpose(),
    ])
}

pub fn add_to_matrix(
    matrix: &mut na::DMatrix<f64>,
    value: &na::Matrix3<f64>,
    start: (usize, usize),
) {
    //Check NaN
    if value[(0, 0)].is_nan()
        || value[(0, 1)].is_nan()
        || value[(0, 2)].is_nan()
        || value[(1, 0)].is_nan()
        || value[(1, 1)].is_nan()
        || value[(1, 2)].is_nan()
        || value[(2, 0)].is_nan()
        || value[(2, 1)].is_nan()
        || value[(2, 2)].is_nan()
    {
        return;
    }
    matrix[(start.0, start.1)] += value[(0, 0)];
    matrix[(start.0, start.1 + 1)] += value[(0, 1)];
    matrix[(start.0, start.1 + 2)] += value[(0, 2)];

    matrix[(start.0 + 1, start.1)] += value[(1, 0)];
    matrix[(start.0 + 1, start.1 + 1)] += value[(1, 1)];
    matrix[(start.0 + 1, start.1 + 2)] += value[(1, 2)];

    matrix[(start.0 + 2, start.1)] += value[(2, 0)];
    matrix[(start.0 + 2, start.1 + 1)] += value[(2, 1)];
    matrix[(start.0 + 2, start.1 + 2)] += value[(2, 2)];
}
