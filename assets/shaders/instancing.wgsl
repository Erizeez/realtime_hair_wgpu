#import bevy_pbr::mesh_functions::{get_model_matrix, mesh_position_local_to_clip}

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,

    @location(3) i_rotation: vec4<f32>,
    @location(4) i_translation: vec3<f32>,
    @location(5) i_scale: vec3<f32>,
    @location(6) i_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

fn quat_to_mat4(q: vec4<f32>) -> mat4x4<f32> {
    let qxx = q.x * q.x;
    let qyy = q.y * q.y;
    let qzz = q.z * q.z;
    let qxz = q.x * q.z;
    let qxy = q.x * q.y;
    let qyz = q.y * q.z;
    let qwx = q.w * q.x;
    let qwy = q.w * q.y;
    let qwz = q.w * q.z;

    var mat : mat4x4<f32>;
    mat[0] = vec4<f32>(
        1.0 - 2.0 * (qyy + qzz),
        2.0 * (qxy + qwz),
        2.0 * (qxz - qwy),
        0.0
    );
    mat[1] = vec4<f32>(
        2.0 * (qxy - qwz),
        1.0 - 2.0 * (qxx + qzz),
        2.0 * (qyz + qwx),
        0.0
    );
    mat[2] = vec4<f32>(
        2.0 * (qxz + qwy),
        2.0 * (qyz - qwx),
        1.0 - 2.0 * (qxx + qyy),
        0.0
    );
    mat[3] = vec4<f32>(0.0, 0.0, 0.0, 1.0);

    return transpose(mat);
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var transformed_position : vec4<f32> = vec4<f32>(vertex.position, 1.0);

    transformed_position.x *= vertex.i_scale.x;
    transformed_position.y *= vertex.i_scale.y;
    transformed_position.z *= vertex.i_scale.z;

    let rotation_matrix = quat_to_mat4(vertex.i_rotation);

    transformed_position = rotation_matrix * transformed_position;

    transformed_position.x += vertex.i_translation.x;
    transformed_position.y += vertex.i_translation.y;
    transformed_position.z += vertex.i_translation.z;

    var out: VertexOutput;
    // NOTE: Passing 0 as the instance_index to get_model_matrix() is a hack
    // for this example as the instance_index builtin would map to the wrong
    // index in the Mesh array. This index could be passed in via another
    // uniform instead but it's unnecessary for the example.
    out.clip_position = mesh_position_local_to_clip(
        get_model_matrix(0u),
        transformed_position
    );
    out.color = vertex.i_color;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}