use ezbuffer::{softbuffer, RED};
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

    let mut time = get_time();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT))
        .with_title("Rotating Cube")
        .build(&event_loop)
        .unwrap();

    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();
    let mut surface = ezbuffer::Surface::new(
        surface,
        (SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize),
        (CANVAS_WIDTH, CANVAS_HEIGHT),
    );
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => {
                // very dirty way to limit FPS, don't actually do this.
                let current_time = get_time();
                let frame_time = current_time - time;
                let min_frame_time = 17;

                if frame_time < min_frame_time {
                    // std::thread::sleep(Duration::from_millis(frame_time - min_frame_time));
                } else {
                    time = current_time;
                    window.request_redraw();
                }
            }

            Event::RedrawRequested(id) if id == window.id() => {
                let (width, height) = {
                    let window_size = window.inner_size();

                    (window_size.width as usize, window_size.height as usize)
                };
                rot_x += 0.01;
                rot_y += 0.05;
                rot_z += 0.03;

                let cube_transform = cube_vertices.map(|(x, y, z)| {
                    let (x, y, z) = rotate_x(x, y, z, rot_x);
                    let (x, y, z) = rotate_y(x, y, z, rot_y);
                    let (x, y, z) = rotate_z(x, y, z, rot_z);

                    let (x, y, z) = ((x + 5.) * 20., (y + 5.) * 20., (z + 5.) * 20.);
                    (x as usize, y as usize, z as usize)
                });

                surface.resize((width, height));
                let mut buffer = surface.buffer();
                buffer.fill(0);

                for (x0, y0, _) in cube_transform {
                    for (x1, y1, _) in cube_transform {
                        buffer.put_line(x0, y0, x1, y1, RED.as_pixel());
                    }
                }

                buffer.present().expect("Couldn't present frame buffer.");
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

fn get_time() -> u128 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let stop = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    stop.as_millis()
}
