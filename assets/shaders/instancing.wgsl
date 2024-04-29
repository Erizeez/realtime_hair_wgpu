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

fn quat_rotate(q: vec4<f32>, rhs: vec3<f32>) -> vec3<f32> {
    let w = q.w;
    let b = q.xyz;
    let b2 = dot(b, b);
    return rhs * (w * w - b2) + 2.0 * dot(b, rhs) * b + 2.0 * w * cross(b, rhs);
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var transformed_position : vec3<f32> = vertex.position;

    transformed_position.x *= vertex.i_scale.x;
    transformed_position.y *= vertex.i_scale.y;
    transformed_position.z *= vertex.i_scale.z;

    transformed_position = quat_rotate(vertex.i_rotation, transformed_position);

    transformed_position += vertex.i_translation;

    var out: VertexOutput;
    // NOTE: Passing 0 as the instance_index to get_model_matrix() is a hack
    // for this example as the instance_index builtin would map to the wrong
    // index in the Mesh array. This index could be passed in via another
    // uniform instead but it's unnecessary for the example.
    out.clip_position = mesh_position_local_to_clip(
        get_model_matrix(0u),
        vec4<f32>(transformed_position, 1.0)
    );
    out.color = vertex.i_color;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}