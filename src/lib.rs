#![no_std]

pub trait Color<T> {
    fn pixel(&self, buf: &mut [T], index: usize) -> T;
}

pub struct Canvas<'a, T> {
    ratio: (f32, f32),
    buf: &'a mut [T],
    surface_size: (usize, usize),
    canvas_size: (usize, usize),
}

fn round(n: f32) -> f32 {
    let nfloor = n as i32 as f32;
    if n - nfloor >= 0.5 {
        nfloor + 1.
    } else {
        nfloor
    }
}

fn modv2(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

impl<'a, T: Clone> Canvas<'a, T> {
    pub fn new(
        buf: &'a mut [T],
        surface_size: (usize, usize),
        canvas_size: (usize, usize),
    ) -> Self {
        let ratio = (
            surface_size.0 as f32 / canvas_size.0 as f32,
            surface_size.1 as f32 / canvas_size.1 as f32,
        );

        Self {
            buf,
            surface_size,
            ratio,
            canvas_size,
        }
    }

    pub fn fill(&mut self, val: T) {
        self.buf.fill(val)
    }

    pub fn clear<C: Color<T>>(&mut self, color: C) {
        let pixel = color.pixel(self.buf, 0);
        self.buf.fill(pixel)
    }

    /// Sets a pixel directly in the surface
    pub fn set_pixel(&mut self, x: usize, y: usize, val: T) {
        self.buf[x + y * self.surface_size.0] = val
    }

    /// Sets a pixel directly in the surface
    pub fn set<C: Color<T>>(&mut self, x: usize, y: usize, color: &C) {
        let idx = x + y * self.surface_size.0;
        self.buf[idx] = color.pixel(self.buf, idx)
    }

    /// 'Put's a color to the specified position on the *canvas*
    pub fn put<C: Color<T>>(&mut self, x: i32, y: i32, color: &C) {
        self.rect(x, y, 1, 1, color);
    }

    /// 'Put's a rectangle to the specified position on the *canvas*
    pub fn rect<C: Color<T>>(&mut self, x: i32, y: i32, w: usize, h: usize, color: &C) {
        #[cfg(not(feature = "wrap"))]
        {
            let x = {
                if x <= 0 {
                    0
                } else {
                    x as usize
                }
            };
            let y = {
                if y <= 0 {
                    0
                } else {
                    y as usize
                }
            };
            for y_idx in round(y as f32 * self.ratio.1) as usize {
                let start = round(x as f32 * self.ratio.0) as usize + y_idx * self.surface_size.0;
                let end =
                    round((x + w) as f32 * self.ratio.0) as usize + y_idx * self.surface_size.0;
                for idx in start..end {
                    if idx < (y_idx + 1) * self.surface_size.0 && idx < self.buf.len() {
                        self.buf[idx] = color.pixel(self.buf, idx);
                    }
                }
            }
        }

        #[cfg(feature = "wrap")]
        {
            let x = modv2(x, self.canvas_size.0 as i32) as usize;
            let y = modv2(y, self.canvas_size.1 as i32) as usize;
            for y_idx in round(y as f32 * self.ratio.1) as usize
                ..round((y + h) as f32 * self.ratio.1) as usize
            {
                for x_idx in round(x as f32 * self.ratio.0) as usize
                    ..round((x + w) as f32 * self.ratio.0) as usize
                {
                    // x_idx and y_idx are `usize` and therefore can't be negative.
                    let x_idx = x_idx % self.surface_size.0;
                    let y_idx = y_idx % self.surface_size.1;
                    let idx = x_idx + y_idx * self.surface_size.0;
                    if idx < (y_idx + 1) * self.surface_size.0 && idx < self.buf.len() {
                        self.buf[idx] = color.pixel(self.buf, idx);
                    }
                }
            }
        }
    }

    /// Draws a line on the canvas using Bresenham's algorithm (no anti aliasing).
    pub fn line<C: Color<T>>(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: &C) {
        // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
        let mut x0 = x0;
        let mut y0 = y0;
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
            self.put(x0, y0, color);
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
    fn pixel(&self, _buf: &mut [u32], _idx: usize) -> u32 {
        match self {
            Self::Rgb(red, green, blue) => {
                ((*red as u32) << 16) | ((*green as u32) << 8) | (*blue as u32)
            }
            Self::Pixel(p) => *p,
        }
    }
}
