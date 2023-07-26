pub struct Canvas<'a> {
    ratio: (usize, usize),
    buf: &'a mut [u32],
    surface_size: (usize, usize),
}

impl<'a> Canvas<'a> {
    pub fn new(
        buf: &'a mut [u32],
        surface_size: (usize, usize),
        canvas_size: (usize, usize),
    ) -> Self {
        let ratio = (
            surface_size.0 / canvas_size.0,
            surface_size.1 / canvas_size.1,
        );

        let ratio = {
            (
                if ratio.0 == 0 { 1 } else { ratio.0 },
                if ratio.1 == 0 { 1 } else { ratio.1 },
            )
        };
        Self {
            buf,
            surface_size,
            ratio,
        }
    }

    pub fn fill(&mut self, val: u32) {
        self.buf.fill(val)
    }

    pub fn clear(&mut self, color: Color) {
        self.buf.fill(color.as_pixel())
    }

    /// Sets a pixel directly in the surface
    pub fn set_pixel(&mut self, x: usize, y: usize, val: u32) {
        self.buf[x + y * self.surface_size.0] = val
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
        for y_idx in y * self.ratio.1..(y + 1) * self.ratio.1 {
            for x_idx in x * self.ratio.0..(x + 1) * self.ratio.0 {
                let idx = x_idx + y_idx * self.surface_size.0;
                if idx < self.buf.len() {
                    self.buf[idx] = val;
                }
            }
        }
    }

    /// Draws a line on the canvas using Bresenham's algorithm (no anti aliasing).
    pub fn put_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: Color) {
        let val = color.as_pixel();
        // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
        let mut x0 = x0 as i32;
        let mut y0 = y0 as i32;
        let x1 = x1 as i32;
        let y1 = y1 as i32;
        let dx = (x1 - x0).abs();
        let sx = {
            if x0 < x1 {
                1
            } else {
                -1
            }
        };
        let dy = -(y1 - y0).abs();
        let sy = {
            if y0 < y1 {
                1
            } else {
                -1
            }
        };
        let mut error = dx + dy;

        loop {
            self.put_pixel(x0 as usize, y0 as usize, val);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * error;
            if e2 >= dy {
                if x0 == x1 {
                    break;
                }
                error += dy;
                x0 += sx;
            }

            if e2 <= dx {
                if y0 == y1 {
                    break;
                }
                error += dx;
                y0 += sy;
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

    /// A method that consumes self and returns the frame buffer which is  mutable u32 slice
    pub fn finish(self) -> &'a mut [u32] {
        self.buf
    }
}

#[derive(Clone)]
pub enum Color {
    Rgb(u8, u8, u8),
    Pixel(u32),
}

pub const RED: Color = Color::Pixel(0xff0000);
pub const GREEN: Color = Color::Pixel(0x00ff00);
pub const BLUE: Color = Color::Pixel(0x0000ff);
pub const WHITE: Color = Color::Pixel(0xffffff);
pub const YELLOW: Color = Color::Pixel(0xffff00);

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
