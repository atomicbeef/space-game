#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}

struct BuildingMaterial {
    grid_color: vec4<f32>,
};

@group(1) @binding(100)
var<uniform> material: BuildingMaterial;

fn grid(uv: vec2<f32>, period: f32, thickness: f32) -> f32 {
    var grid = fract(uv * period);
    var half_width = thickness / 2.0;

    var begin_uv_range_x = step(-half_width, grid.x) - step(grid.x, half_width);
    var begin_uv_range_y = step(-half_width, grid.y) - step(grid.y, half_width);
    var end_uv_range_x = step(period - half_width, grid.x) - step(grid.x, period + half_width);
    var end_uv_range_y = step(period - half_width, grid.y) - step(grid.y, period + half_width);

    return begin_uv_range_x * begin_uv_range_y * end_uv_range_x * end_uv_range_y;
}

@fragment
fn fragment(
    input: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(input, is_front);

    pbr_input.material.base_color = alpha_discard(
        pbr_input.material,
        pbr_input.material.base_color
    );

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);

    var result = 1.0 - grid(input.uv, 1.0, 0.08);

    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    out.color = mix(out.color, material.grid_color, result);

    return out;
}