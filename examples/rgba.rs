use std::num::NonZeroU32;

use framebrush::{Canvas, Draw};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

struct Rgba {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

fn rgba(r: f32, g: f32, b: f32, a: f32) -> Rgba {
    Rgba { r, g, b, a }
}

impl Draw for Rgba {
    type T = u32;
    fn draw(&self, canvas: &mut Canvas<'_, Self::T>, x: i32, y: i32) -> Self::T {
        let prev = *canvas.get(x, y);
        let prev = Rgba {
            r: (prev >> 16) as f32 / 255.,
            g: ((prev >> 8) & 0xff) as f32 / 255.,
            b: (prev & 0xff) as f32 / 255.,
            a: (prev >> 24) as f32 / 255.,
        };
        let blend_a = prev.a + (1. - prev.a) * self.a;
        let blend = Rgba {
            r: blend_a * self.r + (1. - blend_a) * prev.r,
            g: blend_a * self.g + (1. - blend_a) * prev.g,
            b: blend_a * self.b + (1. - blend_a) * prev.b,
            a: blend_a,
        };
        let r = ((blend.r * 255.) as u32) << 16;
        let g = ((blend.g * 255.) as u32) << 8;
        let b = (blend.b * 255.) as u32;
        let a = ((blend.a * 255.) as u32) << 24;

        r | g | b | a
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

                    canvas.rect(10, 10, 30, 30, &rgba(0.85, 0.2, 0., 0.75));

                    canvas.rect(20, 22, 30, 30, &rgba(0.1, 0.2, 0.82, 0.32));

                    canvas.rect(0, 15, 30, 30, &rgba(0.05, 0.9, 0., 0.55));

                    canvas.line(5, 5, 50, 50, &rgba(0.08, 0.85, 0.9, 0.45));

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
