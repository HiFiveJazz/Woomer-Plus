use std::{env, process, time::Instant};

use grim_rs::{CaptureParameters, Grim};
use hyprland::{data::CursorPosition, prelude::*};
use macroquad::prelude::*;
// use macroquad::material::{load_material, MaterialParams};

#[derive(Clone, Copy, Debug)]
struct RectI32 {
    x: i32,
    y: i32,
    w: usize,
    h: usize,
}

const BLUR_VERTEX: &str = r#"
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying vec2 uv;
varying vec4 color;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    uv = texcoord;
    color = color0;
}
"#;

const BLUR_FRAGMENT: &str = r#"
varying vec2 uv;
varying vec4 color;

uniform sampler2D Texture;
uniform vec2 blur_dir;

void main() {
    vec4 sum = vec4(0.0);

    sum += texture2D(Texture, uv - blur_dir * 4.0) * 0.05;
    sum += texture2D(Texture, uv - blur_dir * 3.0) * 0.09;
    sum += texture2D(Texture, uv - blur_dir * 2.0) * 0.12;
    sum += texture2D(Texture, uv - blur_dir * 1.0) * 0.15;
    sum += texture2D(Texture, uv)                * 0.18;
    sum += texture2D(Texture, uv + blur_dir * 1.0) * 0.15;
    sum += texture2D(Texture, uv + blur_dir * 2.0) * 0.12;
    sum += texture2D(Texture, uv + blur_dir * 3.0) * 0.09;
    sum += texture2D(Texture, uv + blur_dir * 4.0) * 0.05;

    gl_FragColor = vec4(sum.rgb * 0.65, 1.0);
}
"#;

fn draw_blurred_background(
    source_texture: &Texture2D,
    blur_target: &RenderTarget,
    blur_material: &mut Material,
    selected_x: f32,
    selected_y: f32,
    selected_w: f32,
    selected_h: f32,
    win_w: f32,
    win_h: f32,
) {
    let src_aspect = selected_w / selected_h;
    let dst_aspect = win_w / win_h;

    let (src_x, src_y, src_w, src_h) = if dst_aspect > src_aspect {
        let new_h = selected_w / dst_aspect;
        let y = selected_y + (selected_h - new_h) * 0.5;
        (selected_x, y, selected_w, new_h)
    } else {
        let new_w = selected_h * dst_aspect;
        let x = selected_x + (selected_w - new_w) * 0.5;
        (x, selected_y, new_w, selected_h)
    };

    // Pass 1: horizontal blur into offscreen target
    set_camera(&Camera2D {
        render_target: Some(blur_target.clone()),
        ..Default::default()
    });
    clear_background(BLACK);

    gl_use_material(blur_material);
    blur_material.set_uniform("blur_dir", [3.0 / selected_w, 0.0]);

    draw_texture_ex(
        source_texture,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(win_w, win_h)),
            source: Some(Rect::new(src_x, src_y, src_w, src_h)),
            ..Default::default()
        },
    );

    gl_use_default_material();

    // Pass 2: vertical blur from offscreen target to screen
    set_default_camera();

    gl_use_material(blur_material);
    blur_material.set_uniform("blur_dir", [0.0, 3.0 / selected_h]);

    draw_texture_ex(
        &blur_target.texture,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(win_w, win_h)),
            source: Some(Rect::new(0.0, 0.0, 1.0, -1.0)),
            ..Default::default()
        },
    );

    gl_use_default_material();
}

fn window_conf() -> Conf {
    Conf {
        window_title: "woomer_plus".to_string(),
        window_width: 1920,
        window_height: 1080,
        high_dpi: true,
        fullscreen: false,
        sample_count: 1,
        window_resizable: false,
        ..Default::default()
    }
}

fn get_initial_cursor_pos_for_output(
    out_x: i32,
    out_y: i32,
    out_w: i32,
    out_h: i32,
) -> Option<(f32, f32)> {
    let pos = CursorPosition::get().ok()?;
    let local_x = pos.x as f32 - out_x as f32;
    let local_y = pos.y as f32 - out_y as f32;

    Some((
        local_x.clamp(0.0, out_w as f32),
        local_y.clamp(0.0, out_h as f32),
    ))
}

fn print_help_and_exit(bin: &str) -> ! {
    eprintln!(
        "\
{bin} – Wayland screen-zoom tool (macroquad prototype)

USAGE:
    {bin} [--monitor <name>]

OPTIONS:
    --monitor <name>   Target monitor (Wayland output name); defaults to first output.",
    );
    process::exit(0);
}
#[macroquad::main(window_conf)]
async fn main() {
    let startup = Instant::now();

    let mut args = env::args();
    let bin = args.next().unwrap();

    let mut monitor_name: Option<String> = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--monitor" => {
                monitor_name = args.next().or_else(|| {
                    eprintln!("--monitor needs a value");
                    process::exit(1);
                })
            }
            _ => print_help_and_exit(&bin),
        }
    }

    let mut grim = Grim::new().expect("failed to initialize grim-rs");
    let outputs = grim.get_outputs().expect("failed to get outputs");

    if outputs.is_empty() {
        eprintln!("No Wayland outputs found.");
        process::exit(1);
    }

    let selected_output = match monitor_name {
        None => &outputs[0],
        Some(ref name) => outputs
            .iter()
            .find(|out| out.name() == name)
            .unwrap_or_else(|| {
                eprintln!("Output '{}' not found.", name);
                process::exit(1);
            }),
    };

    let selected = RectI32 {
        x: selected_output.geometry().x(),
        y: selected_output.geometry().y(),
        w: selected_output.geometry().width() as usize,
        h: selected_output.geometry().height() as usize,
    };

    let params: Vec<CaptureParameters> = outputs
        .iter()
        .map(|out| CaptureParameters::new(out.name()).overlay_cursor(false))
        .collect();

    let results = grim
        .capture_outputs(params)
        .expect("failed to capture outputs");

    let min_x = outputs.iter().map(|o| o.geometry().x()).min().expect("no outputs");
    let min_y = outputs.iter().map(|o| o.geometry().y()).min().expect("no outputs");
    let max_x = outputs
        .iter()
        .map(|o| o.geometry().x() + o.geometry().width() as i32)
        .max()
        .expect("no outputs");
    let max_y = outputs
        .iter()
        .map(|o| o.geometry().y() + o.geometry().height() as i32)
        .max()
        .expect("no outputs");

    let stitched_w = (max_x - min_x) as usize;
    let stitched_h = (max_y - min_y) as usize;
    let mut stitched_rgba = vec![0u8; stitched_w * stitched_h * 4];

    for output in outputs.iter() {
        let result = results
            .get(output.name())
            .expect("missing capture result for output");

        let ox = (output.geometry().x() - min_x) as usize;
        let oy = (output.geometry().y() - min_y) as usize;

        let out_w = result.width() as usize;
        let out_h = result.height() as usize;
        let src = result.data();

        let row_bytes = out_w * 4;
        let dst_stride = stitched_w * 4;

        for row in 0..out_h {
            let dst_start = ((oy + row) * dst_stride) + ox * 4;
            let src_start = row * row_bytes;

            stitched_rgba[dst_start..dst_start + row_bytes]
                .copy_from_slice(&src[src_start..src_start + row_bytes]);
        }
    }

    let texture = Texture2D::from_rgba8(
        stitched_w as u16,
        stitched_h as u16,
        &stitched_rgba,
    );
    texture.set_filter(FilterMode::Nearest);

    let mut blur_material = load_material(
        ShaderSource::Glsl {
            vertex: BLUR_VERTEX,
            fragment: BLUR_FRAGMENT,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("blur_dir", UniformType::Float2),
            ],
            ..Default::default()
        },
    )
    .expect("failed to load blur material");

    let mut camera_target_x = (selected.x - min_x) as f32;
    let mut camera_target_y = (selected.y - min_y) as f32;
    let mut zoom = 1.0f32;
    let mut delta_scale = 0.0f32;

    let mut last_mouse = get_initial_cursor_pos_for_output(
        selected.x,
        selected.y,
        selected.w as i32,
        selected.h as i32,
    )
    .unwrap_or((selected.w as f32 * 0.5, selected.h as f32 * 0.5));

    let mut dragging = false;
    let mut last_drag_mouse = last_mouse;

    request_new_screen_size(selected.w as f32, selected.h as f32);
    let blur_target = render_target(selected.w as u32, selected.h as u32);
    blur_target.texture.set_filter(FilterMode::Linear);

    eprintln!("startup before first present: {:?}", startup.elapsed());

    loop {
        clear_background(BLACK);

        draw_blurred_background(
            &texture,
            &blur_target,
            &mut blur_material,
            (selected.x - min_x) as f32,
            (selected.y - min_y) as f32,
            selected.w as f32,
            selected.h as f32,
            selected.w as f32,
            selected.h as f32,
        );

        let (mx, my) = mouse_position();
        last_mouse = (mx, my);

        let (_scroll_x, scroll_y) = mouse_wheel();
        if scroll_y != 0.0 {
            delta_scale += scroll_y;
        }

        if delta_scale.abs() > 0.01 {
            let pivot_x = last_mouse.0;
            let pivot_y = last_mouse.1;

            let old_zoom = zoom;
            let new_zoom = (zoom + delta_scale * get_frame_time() * 8.0).clamp(1.0, 10.0);

            if (new_zoom - old_zoom).abs() > f32::EPSILON {
                let world_x_before = camera_target_x + pivot_x / old_zoom;
                let world_y_before = camera_target_y + pivot_y / old_zoom;

                zoom = new_zoom;

                camera_target_x = world_x_before - pivot_x / zoom;
                camera_target_y = world_y_before - pivot_y / zoom;
            }

            delta_scale -= delta_scale * get_frame_time() * 8.0;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            dragging = true;
            last_drag_mouse = last_mouse;
        }
        if is_mouse_button_released(MouseButton::Left) {
            dragging = false;
        }

        if dragging {
            let dx = last_mouse.0 - last_drag_mouse.0;
            let dy = last_mouse.1 - last_drag_mouse.1;

            camera_target_x -= dx / zoom;
            camera_target_y -= dy / zoom;

            last_drag_mouse = last_mouse;
        }

        let view_w = selected.w as f32 / zoom;
        let view_h = selected.h as f32 / zoom;

        let src_left = camera_target_x;
        let src_top = camera_target_y;
        let src_right = camera_target_x + view_w;
        let src_bottom = camera_target_y + view_h;

        let tex_w = stitched_w as f32;
        let tex_h = stitched_h as f32;

        let clipped_left = src_left.max(0.0);
        let clipped_top = src_top.max(0.0);
        let clipped_right = src_right.min(tex_w);
        let clipped_bottom = src_bottom.min(tex_h);

        if clipped_right > clipped_left && clipped_bottom > clipped_top {
            let clipped_w = clipped_right - clipped_left;
            let clipped_h = clipped_bottom - clipped_top;

            let dst_x = ((clipped_left - src_left) / view_w) * selected.w as f32;
            let dst_y = ((clipped_top - src_top) / view_h) * selected.h as f32;
            let dst_w = (clipped_w / view_w) * selected.w as f32;
            let dst_h = (clipped_h / view_h) * selected.h as f32;

            draw_texture_ex(
                &texture,
                dst_x,
                dst_y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(dst_w, dst_h)),
                    source: Some(Rect::new(
                        clipped_left,
                        clipped_top,
                        clipped_w,
                        clipped_h,
                    )),
                    ..Default::default()
                },
            );
        }

        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::A) {
            break;
        }

        next_frame().await;
    }
}
