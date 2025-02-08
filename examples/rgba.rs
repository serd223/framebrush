use framebrush::{Canvas, Draw};
use minifb::{Window, WindowOptions};

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
    fn draw(&self, canvas: &mut Canvas<Self::T, &mut [Self::T]>, x: i32, y: i32) {
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

        canvas.put(x, y, r | g | b | a);
    }
}

const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 480;

fn main() {
    let mut buf = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

    let mut window = Window::new(
        "Hello, framebrush!",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions {
            resize: true,
            ..Default::default()
        },
    )
    .unwrap();

    window.set_target_fps(144);
    while window.is_open() {
        let (width, height) = window.get_size();
        buf.resize(width * height, 0);

        // Begin drawing
        let mut canvas = Canvas::new(&mut buf, (width, height), (320, 240));
        canvas.fill(0);

        canvas.rect(10, 10, 30, 30, &rgba(0.85, 0.2, 0., 0.75));
        canvas.rect(20, 22, 30, 30, &rgba(0.1, 0.2, 0.82, 0.32));
        canvas.rect(0, 15, 30, 30, &rgba(0.05, 0.9, 0., 0.55));
        canvas.line(5, 5, 50, 50, &rgba(0.08, 0.85, 0.9, 0.45));

        // End drawing
        window.update_with_buffer(&buf, width, height).unwrap();
    }
}
