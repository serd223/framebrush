use std::num::NonZeroU32;

use framebrush::{Canvas, Color};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

struct RGBA {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color<u32> for RGBA {
    fn pixel(&self, buf: &mut [u32], idx: usize) -> u32 {
        let prev = buf[idx];
        let prev = RGBA {
            r: (prev >> 16) as f32 / 255.,
            g: ((prev >> 8) & 0xff) as f32 / 255.,
            b: (prev & 0xff) as f32 / 255.,
            a: 1.,
        };

        let blend = RGBA {
            r: self.a * self.r + (1. - self.a) * prev.r,
            g: self.a * self.g + (1. - self.a) * prev.g,
            b: self.a * self.b + (1. - self.a) * prev.b,
            a: 1.,
        };
        let r = ((blend.r * 255.) as u32) << 16;
        let g = ((blend.g * 255.) as u32) << 8;
        let b = (blend.b * 255.) as u32;

        r | g | b
    }
}

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT))
        .with_title("RGBA Color Example")
        .build(&event_loop)
        .unwrap();

    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();
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

                    canvas.put_rect(
                        10,
                        10,
                        30,
                        30,
                        &RGBA {
                            r: 0.85,
                            g: 0.2,
                            b: 0.,
                            a: 1.,
                        },
                    );

                    canvas.put_rect(
                        20,
                        22,
                        30,
                        30,
                        &RGBA {
                            r: 0.1,
                            g: 0.2,
                            b: 0.82,
                            a: 0.32,
                        },
                    );

                    canvas.put_rect(
                        0,
                        15,
                        30,
                        30,
                        &RGBA {
                            r: 0.05,
                            g: 0.9,
                            b: 0.,
                            a: 0.55,
                        },
                    );

                    canvas.put_line(
                        5,
                        5,
                        50,
                        50,
                        &RGBA {
                            r: 0.08,
                            g: 0.85,
                            b: 0.9,
                            a: 0.45,
                        },
                    );

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
