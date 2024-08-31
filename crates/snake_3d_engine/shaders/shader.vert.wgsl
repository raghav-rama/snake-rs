[[stage(vertex)]]
fn vs_main([[location(0)]] in_pos: vec3<f32>) -> [[builtin(position)]] vec4<f32> {
    return vec4<f32>(in_pos, 1.0);
}
