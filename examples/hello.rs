use std::num::NonZeroU32;

use framebrush::{Canvas, RGBu32, BLUE, RED, YELLOW};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT))
        .with_title("Hello framebrush!")
        .build(&event_loop)
        .unwrap();

    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();
    let texture: [RGBu32; 16] = [
        BLUE, BLUE, BLUE, BLUE, BLUE, RED, RED, BLUE, BLUE, YELLOW, YELLOW, BLUE, BLUE, BLUE, BLUE,
        BLUE,
    ];
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            }

            Event::RedrawRequested(id) if id == window.id() => {
                let (width, height) = {
                    let window_size = window.inner_size();

                    (window_size.width, window_size.height)
                };
                if let (Some(width_nonzero), Some(height_nonzero)) =
                    (NonZeroU32::new(width), NonZeroU32::new(height))
                {
                    surface.resize(width_nonzero, height_nonzero).unwrap();
                    let mut buffer = surface.buffer_mut().unwrap();

                    let mut canvas =
                        Canvas::new(&mut buffer, (width as usize, height as usize), (320, 240));
                    canvas.fill(0);

                    canvas.put(50, 100, &RED);
                    canvas.put(40, 40, &RGBu32::Rgb(255, 120, 37));
                    canvas.put(60, 60, &RGBu32::Rgb(255, 120, 37));
                    canvas.vert_line(80, 80, 160, &YELLOW);
                    canvas.vert_line(0, 0, 239, &YELLOW);
                    canvas.vert_line(319, 0, 239, &YELLOW);
                    canvas.line(0, 0, 319, 0, &YELLOW);
                    canvas.line(0, 239, 319, 239, &YELLOW);

                    canvas.line(30, 20, 50, 40, &BLUE);

                    canvas.rect(10, 10, 30, 30, &YELLOW);
                    canvas.texture(&texture, 50, 50, (4, 4), (25, 25));
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
