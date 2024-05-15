use std::f32::consts::PI;

use bevy::{
    log::info,
    math::{Quat, Vec3},
    utils::info,
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
    pub position: na::Vector3<f32>,
    pub radius: f32,
    pub rotation: Quat,
    pub attachments: Vec<na::Vector3<f32>>,
}

#[derive(Default, Clone)]
pub struct Hairs {
    pub strands: Vec<HairStrand>,
}

#[derive(Default, Clone, Debug)]
pub struct Frame {
    pub b: na::Vector3<f32>,
    pub n: na::Vector3<f32>,
    pub t: na::Vector3<f32>,
}

#[derive(Clone)]
pub struct HairStrand {
    // Attachment Reference to Head
    pub attachment: usize,

    // Basic properties
    pub radius: f32,
    pub youngs: f32,
    pub shear: f32,

    // Vertices
    pub v_num: usize,
    pub v_mass: Vec<f32>,
    pub v_position: Vec<na::Vector3<f32>>,
    pub v_velocity: Vec<na::Vector3<f32>>,

    // Lines
    pub l_num: usize,
    pub l_momemtum: Vec<f32>,
    pub l_initial_length: Vec<f32>,
    pub l_twist: Vec<f32>,
    pub l_angular: Vec<f32>,
    pub l_initial_kappa: Vec<na::Matrix4x1<f32>>,

    // Reference Frame
    pub reference_frame: Vec<Frame>,
}

pub fn convert_to_vec3(v: na::Vector3<f32>) -> Vec3 {
    Vec3::new(v.x, v.y, v.z)
}

pub fn convert_to_na_vec3(v: Vec3) -> na::Vector3<f32> {
    na::Vector3::new(v.x, v.y, v.z)
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
                translation: strand_translation.into(),
                scale: [1.0, strand_length / HAIR_SEG_LENGTH, 1.0],
                color: [0.27, 0.1, 0.07, 1.0],
            });
        }
        // info!("instance_data: {:?}", &instance_data);
        instance_data
    }

    pub fn get_strand_length(&self, index: usize) -> f32 {
        if index >= self.l_num as usize {
            return -1.0;
        }
        (self.v_position[index + 1] - self.v_position[index]).norm()
    }
}

pub fn generate_straight_hair_strand(
    mass: f32,
    seg_num: usize,
    from_pos: na::Vector3<f32>,
    to_pos: na::Vector3<f32>,
) -> HairStrand {
    let mut hair_strand = HairStrand {
        attachment: 0,
        radius: 0.001,
        youngs: 3000000000.0,
        shear: 1000000000.0,
        v_num: seg_num + 1,
        v_mass: Vec::new(),
        v_position: Vec::new(),
        v_velocity: Vec::new(),
        l_num: seg_num,
        l_momemtum: Vec::new(),
        l_twist: Vec::new(),
        l_angular: Vec::new(),
        l_initial_length: Vec::new(),
        l_initial_kappa: Vec::new(),
        reference_frame: Vec::new(),
    };

    let seg_length = (to_pos - from_pos) / seg_num as f32;

    for i in 0..(seg_num + 1) {
        hair_strand.v_mass.push(mass);
        hair_strand
            .v_position
            .push(from_pos + seg_length * i as f32);
        hair_strand.v_velocity.push(na::Vector3::zeros());
    }

    for i in 0..(seg_num as usize) {
        hair_strand.l_momemtum.push(0.1);

        // Initialize initial length
        hair_strand
            .l_initial_length
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
    center: na::Vector3<f32>,
    radius: f32,
    angle: f32,
    group_num: i32,
    length: f32,
    strand_seg_num: usize,
) -> SimulationData {
    let mut hair_strands = Vec::new();
    let mut head = Head {
        position: center,
        radius,
        rotation: Quat::IDENTITY,
        attachments: Vec::new(),
    };

    let angle_interval = angle / (group_num - 1) as f32;
    let strand_interval = radius * angle_interval as f32;

    for i in 0..group_num {
        let group_angle = i as f32 * angle_interval;
        let mut num: i32 = (2.0 * PI * radius * f32::sin(group_angle) / strand_interval) as i32;
        if num <= 0 {
            num = 1;
        }
        let new_angle_interval = 2.0 * PI / num as f32;
        for j in 0..num {
            let from_strand_pos = na::Vector3::<f32>::new(
                center.x + radius * f32::sin(group_angle) * f32::cos(j as f32 * new_angle_interval),
                center.y + f32::cos(group_angle) * radius,
                center.z + radius * f32::sin(group_angle) * f32::sin(j as f32 * new_angle_interval),
            );
            let to_strand_pos = from_strand_pos + (from_strand_pos - center).normalize() * length;
            let mut hair_strand =
                generate_straight_hair_strand(0.01, strand_seg_num, from_strand_pos, to_strand_pos);

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
