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
    pub hairs: Hairs,
}

#[derive(Default, Clone)]
pub struct Head {
    pub position: Vec3,
    pub radius: f32,
    pub rotation: Quat,
    pub attachments: Vec<Vec3>,
}

#[derive(Default, Clone)]
pub struct Hairs {
    pub strands: Vec<HairStrand>,
    pub youngs: f32,
    pub shear: f32,
}

#[derive(Default, Clone)]
pub struct Frame {
    pub b: Vec3,
    pub n: Vec3,
    pub t: Vec3,
}

#[derive(Clone)]
pub struct HairStrand {
    // Attachment Reference to Head
    pub attachment: usize,

    // Vertices
    pub v_num: i32,
    pub v_mass: Vec<f32>,
    pub v_position: Vec<Vec3>,
    pub v_velocity: Vec<Vec3>,

    // Lines
    pub l_num: i32,
    pub l_momemtum: Vec<f32>,
    pub reference_frame: Vec<Frame>,
}

impl HairStrand {
    pub fn to_instance_data(&self) -> Vec<InstanceData> {
        let mut instance_data = Vec::new();
        for i in 0..(self.v_num - 1) {
            let from_pos = self.v_position[i as usize];
            let to_pos = self.v_position[(i + 1) as usize];

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
    let mut v_mass = Vec::new();
    let mut v_position = Vec::new();
    let mut v_velocity = Vec::new();
    let mut l_momemtum = Vec::new();
    let mut reference_frame = Vec::new();

    let seg_length = (to_pos - from_pos) / seg_num as f32;

    for i in 0..(seg_num + 1) {
        v_mass.push(mass);
        v_position.push(from_pos + seg_length * i as f32);
        v_velocity.push(Vec3::ZERO);
    }

    for i in 0..seg_num {
        l_momemtum.push(0.1);

        // Initialize reference frames
        let e = v_position[(i + 1) as usize] - v_position[i as usize];
        let t = e.normalize();
        if i == 0 {
            let b = t.cross(Vec3::Y).normalize();
            let n = t.cross(b).normalize();
            reference_frame.push(Frame { b, n, t })
        } else {
            let b = reference_frame[i as usize - 1].n.cross(e).normalize();
            let n = t.cross(b).normalize();
            reference_frame.push(Frame { b, n, t })
        }
    }

    HairStrand {
        attachment: 0,
        v_num: seg_num + 1,
        v_mass,
        v_position,
        v_velocity,
        l_num: seg_num,
        l_momemtum,
        reference_frame,
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
                generate_straight_hair_strand(0.01, strand_seg_num, from_strand_pos, to_strand_pos);

            hair_strand.attachment = head.attachments.len();
            head.attachments.push(from_strand_pos - center);
            hair_strands.push(hair_strand);
        }
    }

    info!("hair_strands: {:?}", &hair_strands.len());

    SimulationData {
        hairs: Hairs {
            strands: hair_strands,
            youngs: 3000000000.0,
            shear: 1000000000.0,
        },
        head,
    }
}
