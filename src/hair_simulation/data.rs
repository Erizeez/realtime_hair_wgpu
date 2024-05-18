use std::f64::consts::PI;

use bevy::{
    log::info,
    math::{Quat, Vec3},
    utils::{info, label},
};

use crate::plugins::instanced_mesh::InstanceData;
extern crate nalgebra as na;
use super::{pipeline::der::utils::parallel_transport, HAIR_SEG_LENGTH};

//  Add anything necessary during the simulation HERE.
#[derive(Default, Clone)]
pub struct SimulationData {
    pub head: Head,
    pub hairs: Hairs,
}

#[derive(Default, Clone)]
pub struct Head {
    pub position: na::Vector3<f64>,
    pub radius: f64,
    pub rotation: Quat,
    pub attachments: Vec<na::Vector3<f64>>,
}

#[derive(Default, Clone)]
pub struct Hairs {
    pub strands: Vec<HairStrand>,
}

#[derive(Default, Clone, Debug)]
pub struct Frame {
    pub b: na::Vector3<f64>,
    pub n: na::Vector3<f64>,
    pub t: na::Vector3<f64>,
}

#[derive(Clone)]
pub struct HairStrand {
    // Attachment Reference to Head
    pub attachment: usize,
    pub last_pin: usize,

    // Basic properties
    pub radius: f64,
    pub youngs: f64,
    pub shear: f64,

    // Vertices
    pub v_num: usize,
    pub v_mass: Vec<f64>,
    pub v_position: Vec<na::Vector3<f64>>,
    pub v_velocity: Vec<na::Vector3<f64>>,

    // Lines
    pub l_num: usize,
    pub l_momemtum: Vec<f64>,
    pub l_rest_length: Vec<f64>,
    pub l_twist: Vec<f64>,
    pub l_angular: Vec<f64>,
    pub l_initial_kappa: Vec<na::Matrix4x1<f64>>,

    // Reference Frame
    pub reference_frame: Vec<Frame>,
}

pub fn convert_to_vec3(v: na::Vector3<f64>) -> Vec3 {
    Vec3::new(v.x as f32, v.y as f32, v.z as f32)
}

pub fn convert_to_na_vec3(v: Vec3) -> na::Vector3<f64> {
    na::Vector3::new(v.x as f64, v.y as f64, v.z as f64)
}

impl HairStrand {
    pub fn to_instance_data(&self) -> Vec<InstanceData> {
        let mut instance_data = Vec::new();
        for i in 0..(self.v_num - 1) {
            let from_pos = self.v_position[i as usize];
            let to_pos = self.v_position[(i + 1) as usize];

            let strand_length = (to_pos - from_pos).norm();
            let strand_translation = (from_pos + to_pos) / 2.0;
            let strand_rotation =
                Quat::from_rotation_arc(Vec3::Y, convert_to_vec3((to_pos - from_pos).normalize()));
            // let strand_rotation = Vec3::new(0.0, 1.0, 0.0);

            instance_data.push(InstanceData {
                rotation: strand_rotation.into(),
                translation: [
                    strand_translation.x as f32,
                    strand_translation.y as f32,
                    strand_translation.z as f32,
                ],
                scale: [1.0, (strand_length / HAIR_SEG_LENGTH) as f32, 1.0],
                color: [0.27, 0.1, 0.07, 1.0],
            });
        }
        // info!("instance_data: {:?}", &instance_data);
        instance_data
    }

    pub fn get_strand_length(&self, index: usize) -> f64 {
        if index >= self.l_num as usize {
            return -1.0;
        }
        (self.v_position[index + 1] - self.v_position[index]).norm()
    }
}

pub fn generate_straight_hair_strand(
    mass: f64,
    seg_num: usize,
    from_pos: na::Vector3<f64>,
    to_pos: na::Vector3<f64>,
    youngs: f64,
    shear: f64,
    strand_radius: f64,
    last_pin: usize,
) -> HairStrand {
    let mut hair_strand = HairStrand {
        attachment: 0,
        last_pin,
        radius: strand_radius,
        youngs,
        shear,
        v_num: seg_num + 1,
        v_mass: Vec::new(),
        v_position: Vec::new(),
        v_velocity: Vec::new(),
        l_num: seg_num,
        l_momemtum: Vec::new(),
        l_twist: Vec::new(),
        l_angular: Vec::new(),
        l_rest_length: Vec::new(),
        l_initial_kappa: Vec::new(),
        reference_frame: Vec::new(),
    };

    let seg_length = (to_pos - from_pos) / seg_num as f64;

    for i in 0..(seg_num + 1) {
        hair_strand.v_mass.push(mass);
        hair_strand
            .v_position
            .push(from_pos + seg_length * i as f64);
        hair_strand.v_velocity.push(na::Vector3::zeros());
    }

    for i in 0..(seg_num as usize) {
        hair_strand.l_momemtum.push(0.1);

        // Initialize initial length
        hair_strand
            .l_rest_length
            .push(hair_strand.get_strand_length(i));

        // Initialize twist
        hair_strand.l_twist.push(0.0);

        // Initialize angular
        hair_strand.l_angular.push(0.0);

        // Initialize reference frames
        let t0;
        let t1 = (hair_strand.v_position[i + 1] - hair_strand.v_position[i]).normalize();

        if i == 0 {
            t0 = na::Vector3::new(0.0, 1.0, 0.0);
        } else {
            t0 = hair_strand.reference_frame[i - 1].t;
        }

        let (b, _, n1) = parallel_transport(t0, t1);
        hair_strand.reference_frame.push(Frame { b, n: n1, t: t1 })
    }

    hair_strand
}

pub fn generate_batch_hair_strands(
    center: na::Vector3<f64>,
    radius: f64,
    angle: f64,
    group_num: i32,
    length: f64,
    strand_seg_num: usize,
    youngs: f64,
    shear: f64,
    mass: f64,
    strand_radius: f64,
    last_pin: usize,
) -> SimulationData {
    let mut hair_strands = Vec::new();
    let mut head = Head {
        position: center,
        radius,
        rotation: Quat::IDENTITY,
        attachments: Vec::new(),
    };

    let angle_interval = angle / (group_num - 1) as f64;
    let strand_interval = radius * angle_interval as f64;

    let mass_per_vertex = mass / (strand_seg_num + 1) as f64;

    for i in 2..group_num {
        let group_angle = i as f64 * angle_interval;
        let mut num: i32 = (2.0 * PI * radius * f64::sin(group_angle) / strand_interval) as i32;
        if num <= 0 {
            num = 1;
        }
        let new_angle_interval = 2.0 * PI / num as f64;
        for j in 0..num {
            let from_strand_pos = na::Vector3::<f64>::new(
                center.x + radius * f64::sin(group_angle) * f64::cos(j as f64 * new_angle_interval),
                center.y + f64::cos(group_angle) * radius,
                center.z + radius * f64::sin(group_angle) * f64::sin(j as f64 * new_angle_interval),
            );
            let to_strand_pos = from_strand_pos + (from_strand_pos - center).normalize() * length;
            let mut hair_strand = generate_straight_hair_strand(
                mass_per_vertex,
                strand_seg_num,
                from_strand_pos,
                to_strand_pos,
                youngs,
                shear,
                strand_radius,
                last_pin,
            );

            hair_strand.attachment = head.attachments.len();
            head.attachments.push(from_strand_pos - center);
            hair_strands.push(hair_strand);
        }
        info!("num: {:?}", num);
    }

    info!("hair_strands: {:?}", &hair_strands.len());

    SimulationData {
        hairs: Hairs {
            strands: hair_strands,
        },
        head,
    }
}
