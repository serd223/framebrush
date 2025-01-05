#![no_std]

pub struct Canvas<'a, T> {
    ratio: (f32, f32),
    buf: &'a mut [T],
    surface_size: (usize, usize),
    canvas_size: (usize, usize),
}

pub trait Draw {
    type T;
    fn draw(&self, canvas: &mut Canvas<'_, Self::T>, x: i32, y: i32) -> Self::T;
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

    pub fn clear<D: Draw<T = T>>(&mut self, d: &D) {
        let val = d.draw(self, 0, 0);
        self.fill(val);
    }

    /// Sets a pixel directly in the surface
    pub fn set(&mut self, x: usize, y: usize, val: T) {
        self.buf[x + y * self.surface_size.0] = val
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.buf[x + y * self.surface_size.0]
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.buf[x + y * self.surface_size.0]
    }

    /// Draws any `Drawable` object
    pub fn draw<D: Draw<T = T>>(&mut self, x: i32, y: i32, d: &D) {
        d.draw(self, x, y);
    }

    /// 'Put's a value to the specified position on the *canvas*
    pub fn put(&mut self, x: i32, y: i32, val: T) {
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
            for y_idx in round(y as f32 * self.ratio.1) as usize
                ..round((y + 1) as f32 * self.ratio.1) as usize
            {
                let start = round(x as f32 * self.ratio.0) as usize + y_idx * self.surface_size.0;
                let end =
                    round((x + 1) as f32 * self.ratio.0) as usize + y_idx * self.surface_size.0;
                for idx in start..end {
                    if idx < (y_idx + 1) * self.surface_size.0 && idx < self.buf.len() {
                        self.buf[idx] = val.clone();
                    }
                }
            }
        }

        #[cfg(feature = "wrap")]
        {
            let x = modv2(x, self.canvas_size.0 as i32) as usize;
            let y = modv2(y, self.canvas_size.1 as i32) as usize;
            for y_idx in round(y as f32 * self.ratio.1) as usize
                ..round((y + 1) as f32 * self.ratio.1) as usize
            {
                for x_idx in round(x as f32 * self.ratio.0) as usize
                    ..round((x + 1) as f32 * self.ratio.0) as usize
                {
                    // x_idx and y_idx are `usize` and therefore can't be negative.
                    let x_idx = x_idx % self.surface_size.0;
                    let y_idx = y_idx % self.surface_size.1;
                    let idx = x_idx + y_idx * self.surface_size.0;
                    if idx < (y_idx + 1) * self.surface_size.0 && idx < self.buf.len() {
                        self.set(x_idx, y_idx, val.clone());
                    }
                }
            }
        }
    }

    /// 'Put's a rectangle to the specified position on the *canvas*
    pub fn rect<D: Draw<T = T>>(&mut self, x: i32, y: i32, w: usize, h: usize, d: &D) {
        self.draw(x, y, &Rect { w, h, d });
    }

    /// Draws a line on the canvas using Bresenham's algorithm (no anti aliasing).
    pub fn line<D: Draw<T = T>>(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, d: &D) {
        self.draw(
            x0,
            y0,
            &Line {
                end_x: x1,
                end_y: y1,
                d,
            },
        );
    }

    /// A method that consumes self and returns the frame buffer
    pub fn finish(self) -> &'a mut [T] {
        self.buf
    }
}

pub struct Pixel<T: Clone>(T);
impl<P: Clone> Draw for Pixel<P> {
    type T = P;

    fn draw(&self, canvas: &mut Canvas<'_, Self::T>, x: i32, y: i32) -> Self::T {
        canvas.put(x, y, self.0.clone());
        self.0.clone()
    }
}

pub struct Rect<'a, D: Draw> {
    pub w: usize,
    pub h: usize,
    pub d: &'a D,
}

impl<P: Clone, D: Draw<T = P>> Draw for Rect<'_, D> {
    type T = P;

    fn draw(&self, canvas: &mut Canvas<'_, Self::T>, x: i32, y: i32) -> Self::T {
        let mut y_counter = y;
        let val = self.d.draw(canvas, x, y);
        for _ in 0..self.h {
            let mut x_counter = x;
            for _ in 0..self.w {
                canvas.put(x_counter, y_counter, val.clone());
                x_counter += 1;
            }
            y_counter += 1;
        }
        val
    }
}

struct Line<'a, D: Draw> {
    end_x: i32,
    end_y: i32,
    pub d: &'a D,
}

impl<P: Clone, D: Draw<T = P>> Draw for Line<'_, D> {
    type T = P;

    fn draw(&self, canvas: &mut Canvas<'_, Self::T>, x: i32, y: i32) -> Self::T {
        let val = self.d.draw(canvas, x, y);

        let mut x0 = x;
        let mut y0 = y;
        let x1 = self.end_x;
        let y1 = self.end_y;
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
            canvas.put(x0, y0, val.clone());
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

        val
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

impl Draw for RGBu32 {
    type T = u32;
    fn draw(&self, canvas: &mut Canvas<'_, u32>, x: i32, y: i32) -> u32 {
        let val = match self {
            Self::Rgb(red, green, blue) => {
                ((*red as u32) << 16) | ((*green as u32) << 8) | (*blue as u32)
            }
            RGBu32::Pixel(p) => *p,
        };
        canvas.put(x, y, val);
        val
    }
}
