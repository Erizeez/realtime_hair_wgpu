use std::f32::consts::PI;

use bevy::{
    log::info,
    math::{Quat, Vec3},
};

use crate::plugins::instanced_mesh::InstanceData;

use super::HAIR_SEG_LENGTH;

//  Add anything necessary during the simulation HERE.
#[derive(Default, Clone)]
pub struct SimulationData {
    pub head: Head,
    pub hairs: Vec<HairStrand>,
}

#[derive(Clone, Default)]
pub struct Head {
    pub position: Vec3,
    pub radius: f32,
    pub rotation: Quat,
    pub attachments: Vec<Vec3>,
}

#[derive(Clone)]
pub struct HairStrand {
    pub attachment: usize,
    pub mass: Vec<f32>,
    pub position: Vec<Vec3>,
    pub velocity: Vec<Vec3>,
    pub acceleration: Vec<Vec3>,
    pub num: i32,
}

impl HairStrand {
    pub fn to_instance_data(&self) -> Vec<InstanceData> {
        let mut instance_data = Vec::new();
        for i in 0..(self.num - 1) {
            let from_pos = self.position[i as usize];
            let to_pos = self.position[(i + 1) as usize];

            let strand_length = (to_pos - from_pos).length();
            let strand_translation = (from_pos + to_pos) / 2.0;
            let strand_rotation = Quat::from_rotation_arc(Vec3::Y, (to_pos - from_pos).normalize());
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
}

pub fn generate_straight_hair_strand(
    mass: f32,
    seg_num: i32,
    from_pos: Vec3,
    to_pos: Vec3,
) -> HairStrand {
    let mut mass_vec = Vec::new();
    let mut pos_vec = Vec::new();
    let mut vel_vec = Vec::new();
    let mut acc_vec = Vec::new();

    let seg_length = (to_pos - from_pos) / seg_num as f32;

    for i in 0..(seg_num + 1) {
        mass_vec.push(mass);
        pos_vec.push(from_pos + seg_length * i as f32);
        vel_vec.push(Vec3::ZERO);
        acc_vec.push(Vec3::ZERO);
    }

    HairStrand {
        attachment: 0,
        mass: mass_vec,
        position: pos_vec,
        velocity: vel_vec,
        acceleration: acc_vec,
        num: seg_num + 1,
    }
}

pub fn generate_batch_hair_strands(
    center: Vec3,
    radius: f32,
    angle: f32,
    group_num: i32,
    length: f32,
    strand_seg_num: i32,
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
        let mut num = (2.0 * PI / strand_interval) as i32;
        if num <= 0 {
            num = 1;
        }
        for j in 0..num {
            let from_strand_pos = Vec3::new(
                center.x + radius * f32::sin(group_angle) * f32::cos(j as f32 * angle_interval),
                center.y + f32::cos(group_angle) * radius,
                center.z + radius * f32::sin(group_angle) * f32::sin(j as f32 * angle_interval),
            );
            let to_strand_pos = from_strand_pos + (from_strand_pos - center).normalize() * length;
            let mut hair_strand =
                generate_straight_hair_strand(1.0, strand_seg_num, from_strand_pos, to_strand_pos);

            hair_strand.attachment = head.attachments.len();
            head.attachments.push(from_strand_pos - center);
            hair_strands.push(hair_strand);
        }
    }

    info!("hair_strands: {:?}", &hair_strands.len());

    SimulationData {
        hairs: hair_strands,
        head,
    }
}
