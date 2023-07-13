use std::num::NonZeroU32;

pub use softbuffer;

pub struct Surface {
    surface: softbuffer::Surface,
    surface_size: (usize, usize),
    canvas_size: (usize, usize),
}

impl Surface {
    pub fn new(
        surface: softbuffer::Surface,
        surface_size: (usize, usize),
        canvas_size: (usize, usize),
    ) -> Self {
        Self {
            surface,
            surface_size,
            canvas_size,
        }
    }

    pub fn resize(&mut self, surface_size: (usize, usize)) {
        self.surface
            .resize(
                NonZeroU32::new(surface_size.0 as u32).unwrap(),
                NonZeroU32::new(surface_size.1 as u32).unwrap(),
            )
            .expect("Couldn't resize surface.");
        self.surface_size = surface_size;
    }

    pub fn resize_canvas(&mut self, canvas_size: (usize, usize)) {
        self.canvas_size = canvas_size;
    }

    pub fn buffer(&mut self) -> Buffer<'_> {
        let buffer = self.surface.buffer_mut().unwrap();

        Buffer {
            inner: buffer,
            surface_size: (self.surface_size.0, self.surface_size.1),
            ratio: (
                self.surface_size.0 / self.canvas_size.0,
                self.surface_size.1 / self.canvas_size.1,
            ),
        }
    }
}

pub struct Buffer<'a> {
    inner: softbuffer::Buffer<'a>,
    surface_size: (usize, usize),
    ratio: (usize, usize),
}

impl<'a> Buffer<'a> {
    pub fn fill(&mut self, val: u32) {
        self.inner.fill(val)
    }

    pub fn clear(&mut self, color: Color) {
        self.inner.fill(color.as_pixel())
    }

    /// Sets a pixel directly in the surface
    pub fn set_pixel(&mut self, x: usize, y: usize, val: u32) {
        self.inner[x + y * self.surface_size.0] = val
    }

    /// Sets a pixel directly in the surface
    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        self.set_pixel(x, y, color.as_pixel())
    }

    /// 'Put's a color to the specified position on the *canvas*
    pub fn put(&mut self, x: usize, y: usize, color: Color) {
        self.put_pixel(x, y, color.as_pixel());
    }

    /// 'Put's a pixel to the specified position on the *canvas*
    pub fn put_pixel(&mut self, x: usize, y: usize, val: u32) {
        if matches!(self.ratio, (0, 0) | (1, 1)) {
            let idx = self.surface_size.0 * y + x;
            if idx < self.inner.len() {
                self.inner[idx] = val;
            }
            return;
        }
        let ratio = (
            if self.ratio.0 > 0 { self.ratio.0 } else { 1 },
            if self.ratio.1 > 0 { self.ratio.1 } else { 1 },
        );
        for y_idx in y * ratio.1..(y + 1) * ratio.1 {
            for x_idx in x * ratio.0..(x + 1) * ratio.0 {
                let idx = x_idx + y_idx * self.surface_size.0;
                if idx < self.inner.len() {
                    self.inner[idx] = val;
                }
            }
        }
    }

    /// Simple convenience function for drawing vertical lines on the canvas.
    pub fn vert_line(&mut self, x: usize, draw_start: usize, draw_end: usize, color: Color) {
        let start = draw_start.min(draw_end);
        let end = draw_start.max(draw_end);
        let pixel = color.as_pixel();
        for y in start..end + 1 {
            self.put_pixel(x, y, pixel);
        }
    }

    pub fn present(self) -> Result<(), softbuffer::SoftBufferError> {
        self.inner.present()
    }
}

#[derive(Clone)]
pub enum Color {
    Rgb(u8, u8, u8),
    Pixel(u32),
}

pub const RED: Color = Color::Rgb(255, 0, 0);
pub const GREEN: Color = Color::Rgb(0, 255, 0);
pub const BLUE: Color = Color::Rgb(0, 0, 255);
pub const WHITE: Color = Color::Rgb(255, 255, 255);
pub const YELLOW: Color = Color::Rgb(255, 255, 0);

impl Color {
    pub fn as_pixel(&self) -> u32 {
        match self {
            Self::Rgb(red, green, blue) => {
                ((*red as u32) << 16) | ((*green as u32) << 8) | (*blue as u32)
            }
            Self::Pixel(p) => *p,
        }
    }
}
