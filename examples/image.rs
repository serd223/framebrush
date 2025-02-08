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

use framebrush::{Canvas, Draw};
use image::IMAGE;
use minifb::{Window, WindowOptions};
use std::marker::PhantomData;

const DEFAULT_WIDTH: usize = 800;
const DEFAULT_HEIGHT: usize = 600;

// T is AsRef<[U]> instead of something simple like Vec<U> because we want ImageSource to be
// able to both own or just hold a reference to the source data. For instance, we probably don't
// need to own the asset data we read from the disk or from some other source, but we should own
// the data that we get after rendering the image to our desired size. So in this example we only
// provide a slice of `IMAGE` to ImageSource but then we render that image which owns the new rendered data
// and that rendered data is stored in the `image_render` variable
struct ImageSource<T: AsRef<[U]>, U: Clone> {
    data: T,
    width: usize,
    _marker: PhantomData<U>, // Needed because rust
}

impl<U: Clone, T: AsRef<[U]>> ImageSource<T, U> {
    fn new(data: T, width: usize) -> Self {
        Self {
            data,
            width,
            _marker: PhantomData,
        }
    }

    // We panic on an empty image because we don't have access to something like U::default() so we can't fill the resulting image
    // with some default value
    fn render(&self, target_width: usize, target_height: usize) -> ImageSource<Vec<U>, U> {
        // Just copy the existing data instead of using something like U::default() because we want to be as generic as possible
        let mut render_data = vec![
            self.data
                .as_ref()
                .get(0)
                .expect("can't render empty image")
                .clone();
            target_width * target_height
        ];

        // Use the framebrush Canvas to resize our image correctly instead of implementing the same functionality again
        let mut render_canvas = Canvas::new(
            &mut render_data,
            (target_width, target_height),
            (self.width, self.data.as_ref().len() / self.width),
        );
        // The imaginary canvas is the same size as our original image. We can imagine the render_data as the framebuffer
        // of our screen and the size of our original image can be imagined as the resolution of your game, for instance.
        // We are basically treating the rendered image as a window and using framebrush's scaling implementation to avoid
        // wiriting the same logic again.
        render_canvas.draw(0, 0, self);
        ImageSource::new(render_data, target_width)
    }
}

impl<U: Clone, T: AsRef<[U]>> Draw for ImageSource<T, U> {
    type T = U;
    fn draw(&self, canvas: &mut Canvas<Self::T, &mut [Self::T]>, start_x: i32, start_y: i32) {
        for (y, strip) in self.data.as_ref().chunks(self.width).enumerate() {
            for (x, c) in strip.iter().enumerate() {
                canvas.put(start_x + x as i32, start_y + y as i32, c.clone());
            }
        }
    }
}

fn main() {
    let mut framebuffer = vec![0; DEFAULT_WIDTH * DEFAULT_HEIGHT];

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

    let image_render = ImageSource::new(&IMAGE, 8).render(200, 300);

    window.set_target_fps(144);
    while window.is_open() {
        let (width, height) = window.get_size();
        framebuffer.resize(width * height, 0);

        // Begin drawing
        let mut canvas = Canvas::new(
            &mut framebuffer,
            (width, height),
            (DEFAULT_WIDTH, DEFAULT_HEIGHT),
        );
        canvas.fill(0);
        canvas.draw(100, 100, &image_render);

        // End drawing
        window
            .update_with_buffer(&framebuffer, width, height)
            .unwrap();
    }
}
