use std::time::{Duration, Instant};

use ezbuffer::{softbuffer, BLUE};
use ezbuffer::{Color, RED, YELLOW};
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
        .with_title("Hello ezbuffer!")
        .build(&event_loop)
        .unwrap();

    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();
    let mut surface = ezbuffer::Surface::new(
        surface,
        (SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize),
        (320, 240),
    );
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(
            Instant::now()
                .checked_add(Duration::from_micros(1_000_000 / 60))
                .unwrap(),
        );

        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            }

            Event::RedrawRequested(id) if id == window.id() => {
                let (width, height) = {
                    let window_size = window.inner_size();

                    (window_size.width as usize, window_size.height as usize)
                };
                surface.resize((width, height));
                let mut buffer = surface.buffer();
                buffer.fill(0);

                buffer.put(50, 100, RED);
                buffer.put_pixel(100, 50, 0x001f1fff);
                buffer.put(40, 40, Color::Rgb(255, 120, 37));
                buffer.put(60, 60, Color::Pixel(0x00ffffff));

                buffer.vert_line(80, 80, 160, YELLOW);

                buffer.put_line(30, 20, 50, 40, BLUE.as_pixel());

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
