#![no_std]

pub trait Color<T> {
    fn pixel(&self, buf: &mut [T], index: usize) -> T;
}

pub struct Canvas<'a, T> {
    ratio: (f32, f32),
    buf: &'a mut [T],
    surface_size: (usize, usize),
}

fn round(n: f32) -> f32 {
    let nfloor = n as i32 as f32;
    if n - nfloor >= 0.5 {
        nfloor + 1.
    } else {
        nfloor
    }
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
    pub fn put<C: Color<T>>(&mut self, x: usize, y: usize, color: &C) {
        self.rect(x, y, 1, 1, color);
    }

    /// 'Put's a rectangle to the specified position on the *canvas*
    pub fn rect<C: Color<T>>(&mut self, x: usize, y: usize, w: usize, h: usize, color: &C) {
        // let slice_len = w as f32 * self.ratio.0;
        // let len = self.buf.len();
        // let horizontal_slice = std::vec![val; slice_len];
        for y_idx in
            round(y as f32 * self.ratio.1) as usize..round((y + h) as f32 * self.ratio.1) as usize
        {
            let start = round(x as f32 * self.ratio.0) as usize + y_idx * self.surface_size.0;
            let end = round((x + w) as f32 * self.ratio.0) as usize + y_idx * self.surface_size.0;
            for idx in start..end {
                if idx < (y_idx + 1) * self.surface_size.0 && idx < self.buf.len() {
                    self.buf[idx] = color.pixel(self.buf, idx);
                }
            }
            // This seemed like a cool little optimization but it turned out to have some issues.
            // For example, if you are using RGBA colors with the Alpha channel, you will need access
            // to the previously existing pixel in the position that you want to draw on. That requires
            // access to every single individual pixel.
            // With the method below, you don't access individual pixels, which makes it pretty limiting.
            // if start < len {
            //     if end < len {
            //         self.buf[start..end].clone_from_slice(&horizontal_slice);
            //     } else {
            //         let slice_len = len - start;
            //         self.buf[start..].clone_from_slice(&horizontal_slice[..slice_len]);
            //     }
            // }
        }
    }

    pub fn texture<C: Color<T>>(
        &mut self,
        texture_data: &[C],
        x: usize,
        y: usize,
        source_size: (usize, usize),
        dest_size: (usize, usize),
    ) {
        let (sw, sh) = source_size;
        let (dw, dh) = dest_size;
        let y_start = round(y as f32 * self.ratio.1) as usize;
        let y_end = round((y + dh) as f32 * self.ratio.1) as usize;
        let mut iy = 0;
        for y_idx in y_start..y_end {
            let start = round(x as f32 * self.ratio.0) as usize + y_idx * self.surface_size.0;
            let end = round((x + dw) as f32 * self.ratio.0) as usize + y_idx * self.surface_size.0;
            let mut ix = 0;
            for idx in start..end {
                if idx < (y_idx + 1) * self.surface_size.0 && idx < self.buf.len() {
                    let c_idx = ((ix as f32) / ((end - start) as f32) * (sw as f32)) as usize
                        + ((iy as f32) / ((y_end - y_start) as f32) * (sh as f32)) as usize * sh;
                    self.buf[idx] = texture_data[c_idx].pixel(self.buf, idx);
                }
                ix += 1;
            }
            iy += 1;
        }
    }

    /// Draws a line on the canvas using Bresenham's algorithm (no anti aliasing).
    pub fn line<C: Color<T>>(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: &C) {
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
            self.put(x0 as usize, y0 as usize, color);
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
        color: &C,
    ) {
        let start = draw_start.min(draw_end);
        let end = draw_start.max(draw_end);
        for y in start..end + 1 {
            self.put(x, y, color);
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
