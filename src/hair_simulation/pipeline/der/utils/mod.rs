use crate::hair_simulation::data::Frame;
extern crate nalgebra as na;

pub fn parallel_transport(
    t0: na::Vector3<f32>,
    t1: na::Vector3<f32>,
) -> (na::Vector3<f32>, na::Vector3<f32>, na::Vector3<f32>) {
    let b = t0.cross(&t1);
    let n0 = t0.cross(&b);
    let n1 = t1.cross(&b);

    (b, n0.normalize(), n1.normalize())
}

pub fn partial_kappa(
    e_vec: &Vec<na::Vector3<f32>>,
    t_tilde: &na::Vector3<f32>,
    kappa: &Vec<na::Matrix4x1<f32>>,
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
) -> na::Matrix3x1<f32> {
    let kappa_up_index;

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

    1.0 / e.norm() * (kappa_part + latter_sign as f32 * latter_part)
}
