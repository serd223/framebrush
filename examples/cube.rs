use framebrush::{Canvas, RED};
use minifb::{Window, WindowOptions};
use std::time::Instant;

const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 480;

const CANVAS_WIDTH: usize = 320;
const CANVAS_HEIGHT: usize = 240;

fn main() {
    let cube_vertices = [
        (-1., -1., -1.),
        (1., -1., -1.),
        (-1., -1., 1.),
        (1., -1., 1.),
        (-1., 1., -1.),
        (1., 1., -1.),
        (-1., 1., 1.),
        (1., 1., 1.),
    ];
    let (mut rot_x, mut rot_y, mut rot_z) = (0., 0., 0.);

    let mut buf = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];
    let mut window = Window::new(
        "Rotating Cube",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions {
            resize: true,
            ..Default::default()
        },
    )
    .unwrap();

    window.set_target_fps(144);
    let mut last_frame = Instant::now();
    while window.is_open() {
        let delta = {
            let now = Instant::now();
            let res = now.duration_since(last_frame).as_secs_f32();
            last_frame = now;
            res
        };

        let (width, height) = window.get_size();
        buf.resize(width * height, 0);

        rot_x += 1. * delta;
        rot_y += 5. * delta;
        rot_z += 3. * delta;

        let cube_transform = cube_vertices.map(|(x, y, z)| {
            let (x, y, z) = rotate_x(x, y, z, rot_x);
            let (x, y, z) = rotate_y(x, y, z, rot_y);
            let (x, y, z) = rotate_z(x, y, z, rot_z);

            let (x, y, z) = ((x + 5.) * 20., (y + 5.) * 20., (z + 5.) * 20.);
            (x as i32, y as i32, z as i32)
        });

        let mut canvas = Canvas::new(&mut buf, (width, height), (CANVAS_WIDTH, CANVAS_HEIGHT));
        canvas.fill(0);

        for (x0, y0, _) in cube_transform {
            for (x1, y1, _) in cube_transform {
                canvas.line(x0, y0, x1, y1, &RED);
            }
        }

        window.update_with_buffer(&mut buf, width, height).unwrap();
    }
}

pub fn rotate_x(x: f32, y: f32, z: f32, rot: f32) -> (f32, f32, f32) {
    (
        x,
        (y * rot.cos()) - (z * rot.sin()),
        (y * rot.sin()) + (z * rot.cos()),
    )
}
pub fn rotate_y(x: f32, y: f32, z: f32, rot: f32) -> (f32, f32, f32) {
    (
        (z * rot.sin()) + (x * rot.cos()),
        y,
        (z * rot.cos()) - (x * rot.sin()),
    )
}
pub fn rotate_z(x: f32, y: f32, z: f32, rot: f32) -> (f32, f32, f32) {
    (
        (x * rot.cos()) - (y * rot.sin()),
        (x * rot.sin()) + (y * rot.cos()),
        z,
    )
}
