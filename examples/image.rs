mod image {
    const BLACK: u32 = 0;
    const RED: u32 = 0xff0000;
    const GREEN: u32 = 0x00ff00;
    const BLUE: u32 = 0x0000ff;
    const WHITE: u32 = 0xffffff;

    #[rustfmt::skip]
    pub const IMAGE: [u32; 64] = [
        RED,   RED, BLUE, RED, RED,   RED,   RED,   BLACK,
        WHITE, RED, RED,  RED, RED,   RED,   BLACK, RED,
        WHITE, RED, RED,  RED, GREEN, GREEN, RED,   RED,
        WHITE, RED, RED,  RED, GREEN, RED,   RED,   RED,
        WHITE, RED, RED,  RED, GREEN, RED,   RED,   RED,
        WHITE, RED, BLUE, RED, RED,   RED,   RED,   RED,
        WHITE, RED, RED,  RED, RED,   RED,   RED,   BLUE,
        WHITE, RED, RED,  RED, RED,   RED,   BLUE,  GREEN,
    ];
}
use image::IMAGE;

use framebrush::{Canvas, Draw};
use minifb::{Window, WindowOptions};

const DEFAULT_WIDTH: usize = 800;
const DEFAULT_HEIGHT: usize = 600;

struct ImageSource<T: AsRef<[u32]>> {
    data: T,
    width: usize,
}

impl<T: AsRef<[u32]>> ImageSource<T> {
    fn render(&self, target_width: usize, target_height: usize) -> ImageSource<Vec<u32>> {
        let mut res = vec![0; target_width * target_height];
        let mut canvas = Canvas::new(
            &mut res,
            (target_width, target_height),
            (self.width, self.data.as_ref().len() / self.width),
        );
        self.draw(&mut canvas, 0, 0);
        ImageSource {
            data: res,
            width: target_width,
        }
    }
}

impl<T: AsRef<[u32]>> Draw for ImageSource<T> {
    type T = u32;
    fn draw(&self, canvas: &mut Canvas<u32>, start_x: i32, start_y: i32) -> u32 {
        for (y, strip) in self.data.as_ref().chunks(self.width).enumerate() {
            for (x, c) in strip.iter().enumerate() {
                canvas.put(start_x + x as i32, start_y + y as i32, *c);
            }
        }
        0
    }
}

fn main() {
    let mut buf = vec![0 as u32; DEFAULT_WIDTH * DEFAULT_HEIGHT];

    let mut window = Window::new(
        "Image Example",
        DEFAULT_WIDTH,
        DEFAULT_HEIGHT,
        WindowOptions {
            resize: true,
            ..Default::default()
        },
    )
    .unwrap();

    let image_render = ImageSource {
        data: IMAGE,
        width: 8,
    }
    .render(200, 300);

    window.set_target_fps(144);
    while window.is_open() {
        let (width, height) = window.get_size();
        buf.resize(width * height, 0);

        // Begin drawing
        let mut canvas = Canvas::new(&mut buf, (width, height), (DEFAULT_WIDTH, DEFAULT_HEIGHT));
        canvas.fill(0);
        canvas.draw(100, 100, &image_render);

        // End drawing
        window.update_with_buffer(&buf, width, height).unwrap();
    }
}
