
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>
}

@group(1) @binding(1)
var _texture: texture_2d<f32>;

@group(1) @binding(2)
var _sampler: sampler;

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    var offset = 0.01;

    var output = vec4<f32>(input.uv, 0.0, 1.0);
    var tex_sample = textureSample(_texture, _sampler, input.uv);

    var diff_0 = textureSample(_texture, _sampler, input.uv+vec2<f32>(offset, offset)).w - tex_sample.w;
    var diff_1 = textureSample(_texture, _sampler, input.uv+vec2<f32>(-offset, offset)).w - tex_sample.w;
    var diff_2 = textureSample(_texture, _sampler, input.uv+vec2<f32>(-offset, -offset)).w - tex_sample.w;
    var diff_3 = textureSample(_texture, _sampler, input.uv+vec2<f32>(offset, -offset)).w - tex_sample.w;

    var diff = clamp(diff_0 + diff_1 + diff_2 + diff_3, 0.0, 1.0);
    output = output * diff;
    output = output + tex_sample;
    return output;
}
