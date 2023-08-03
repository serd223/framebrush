use framebrush::{Canvas, RED};
use std::{
    num::NonZeroU32,
    time::{Duration, Instant},
};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

const CANVAS_WIDTH: usize = 320;
const CANVAS_HEIGHT: usize = 240;

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

    let mut last_redraw = Instant::now();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT))
        .with_title("Rotating Cube")
        .build(&event_loop)
        .unwrap();

    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(
            Instant::now()
                .checked_add(Duration::from_micros(1_000_000 / 144))
                .unwrap(),
        );

        match event {
            Event::MainEventsCleared => {
                let current_time = Instant::now();
                let frame_time = current_time - last_redraw;
                let min_frame_time = 17000;

                if frame_time.as_micros() > min_frame_time {
                    let delta_time = frame_time.as_secs_f32();
                    rot_x += 1. * delta_time;
                    rot_y += 5. * delta_time;
                    rot_z += 3. * delta_time;
                    last_redraw = current_time;
                    window.request_redraw();
                }
            }

            Event::RedrawRequested(id) if id == window.id() => {
                let (width, height) = {
                    let window_size = window.inner_size();

                    (window_size.width, window_size.height)
                };

                let cube_transform = cube_vertices.map(|(x, y, z)| {
                    let (x, y, z) = rotate_x(x, y, z, rot_x);
                    let (x, y, z) = rotate_y(x, y, z, rot_y);
                    let (x, y, z) = rotate_z(x, y, z, rot_z);

                    let (x, y, z) = ((x + 5.) * 20., (y + 5.) * 20., (z + 5.) * 20.);
                    (x as usize, y as usize, z as usize)
                });

                if let (Some(width_nonzero), Some(height_nonzero)) =
                    (NonZeroU32::new(width), NonZeroU32::new(height))
                {
                    surface.resize(width_nonzero, height_nonzero).unwrap();
                    let mut buffer = surface.buffer_mut().unwrap();
                    let mut canvas = Canvas::new(
                        &mut buffer,
                        (width as usize, height as usize),
                        (CANVAS_WIDTH, CANVAS_HEIGHT),
                    );
                    canvas.fill(0);

                    for (x0, y0, _) in cube_transform {
                        for (x1, y1, _) in cube_transform {
                            canvas.line(x0, y0, x1, y1, &RED);
                        }
                    }

                    buffer.present().expect("Couldn't present frame buffer.");
                }
            }

            Event::WindowEvent {
                window_id: id,
                event: WindowEvent::CloseRequested,
            } if id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }

            _ => (),
        }
    });
}
