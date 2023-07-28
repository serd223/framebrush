pub trait Color<T> {
    fn pixel(&self) -> T;
}

pub struct Canvas<'a, T: Clone> {
    ratio: (usize, usize),
    buf: &'a mut [T],
    surface_size: (usize, usize),
}

impl<'a, T: Clone> Canvas<'a, T> {
    pub fn new(
        buf: &'a mut [T],
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

    pub fn fill(&mut self, val: T) {
        self.buf.fill(val)
    }

    pub fn clear<C: Color<T>>(&mut self, color: C) {
        self.buf.fill(color.pixel())
    }

    /// Sets a pixel directly in the surface
    pub fn set_pixel(&mut self, x: usize, y: usize, val: T) {
        self.buf[x + y * self.surface_size.0] = val
    }

    /// Sets a pixel directly in the surface
    pub fn set<C: Color<T>>(&mut self, x: usize, y: usize, color: C) {
        self.set_pixel(x, y, color.pixel())
    }

    /// 'Put's a color to the specified position on the *canvas*
    pub fn put<C: Color<T>>(&mut self, x: usize, y: usize, color: C) {
        self.put_pixel(x, y, color.pixel());
    }

    /// 'Put's a pixel to the specified position on the *canvas*
    pub fn put_pixel(&mut self, x: usize, y: usize, val: T) {
        self.put_rect(x, y, 1, 1, val);
    }

    /// 'Put's a rectangle to the specified position on the *canvas*
    pub fn put_rect(&mut self, x: usize, y: usize, w: usize, h: usize, val: T) {
        let len = self.buf.len();
        let slice_len = w * self.ratio.0;
        let horizontal_slice = vec![val; slice_len];
        for y_idx in y * self.ratio.1..(y + h) * self.ratio.1 {
            let start = x * self.ratio.0 + y_idx * self.surface_size.0;
            let end = start + slice_len;
            if start < len {
                if end < len {
                    self.buf[start..end].clone_from_slice(&horizontal_slice);
                } else {
                    let slice_len = len - start;
                    self.buf[start..].clone_from_slice(&horizontal_slice[..slice_len]);
                }
            }
        }
    }

    /// Draws a line on the canvas using Bresenham's algorithm (no anti aliasing).
    pub fn put_line<C: Color<T>>(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: C) {
        let val = color.pixel();
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
            self.put_pixel(x0 as usize, y0 as usize, val.clone());
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
    pub fn vert_line<C: Color<T>>(
        &mut self,
        x: usize,
        draw_start: usize,
        draw_end: usize,
        color: C,
    ) {
        let start = draw_start.min(draw_end);
        let end = draw_start.max(draw_end);
        let pixel = color.pixel();
        for y in start..end + 1 {
            self.put_pixel(x, y, pixel.clone());
        }
    }

    /// A method that consumes self and returns the frame buffer
    pub fn finish(self) -> &'a mut [T] {
        self.buf
    }
}

#[derive(Clone)]
/// The Rgb variant is converted to the 00000000RRRRRRRRGGGGGGGGBBBBBBBB format when .as_pixel() is called, for custom pixel formats, use the Pixel variant.
pub enum RGBu32 {
    Rgb(u8, u8, u8),
    Pixel(u32),
}

pub const RED: RGBu32 = RGBu32::Pixel(0xff0000);
pub const GREEN: RGBu32 = RGBu32::Pixel(0x00ff00);
pub const BLUE: RGBu32 = RGBu32::Pixel(0x0000ff);
pub const WHITE: RGBu32 = RGBu32::Pixel(0xffffff);
pub const YELLOW: RGBu32 = RGBu32::Pixel(0xffff00);

impl Color<u32> for RGBu32 {
    fn pixel(&self) -> u32 {
        match self {
            Self::Rgb(red, green, blue) => {
                ((*red as u32) << 16) | ((*green as u32) << 8) | (*blue as u32)
            }
            Self::Pixel(p) => *p,
        }
    }
}
