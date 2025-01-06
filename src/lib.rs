#![no_std]

/// Main entry point of `framebrush`, a `Canvas` can be constructed with `Canvas::new`. `Canvas::new` doesn't perform any allocations and only does some calculations for resizing. You don't have to reconstruct your `Canvas` if you don't plan on resizing your framebuffer however it is still recommended to create a new `Canvas` each frame to avoid ownership issues regarding the mutable slice `buf`.
pub struct Canvas<'a, T> {
    ratio: (f32, f32),
    pub buf: &'a mut [T],
    surface_size: (usize, usize),
    #[allow(dead_code)]
    // unused when the 'wrap' feature is disabled but i decided to keep it just in case
    // the field is used in a function that is independent from the 'wrap' feature in the future.
    canvas_size: (usize, usize),
}

/// Trait for any `draw`able object, ranging from shapes like `Rect`, `Line` or even `Pixel` to colors. The `Draw` API is designed to be as generic as possible to make its usage easy in any context
pub trait Draw {
    type T;
    fn draw(&self, canvas: &mut Canvas<'_, Self::T>, canvas_x: i32, canvas_y: i32);
}

fn round(n: f32) -> f32 {
    let nfloor = n as i32 as f32;
    if n - nfloor >= 0.5 {
        nfloor + 1.
    } else {
        nfloor
    }
}

#[cfg(feature = "wrap")]
fn modv2(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

impl<'a, T: Clone> Canvas<'a, T> {
    /// Creates new `Canvas` with specified parameters
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

    /// `fill`s the entire buffer of the `Canvas` with a value of type `T`
    pub fn fill(&mut self, val: T) {
        self.buf.fill(val)
    }

    /// `clear`s the `Canvas` by calling the `.draw` method of `d` at (0, 0) and `fill`ing the canvas with the return value.
    pub fn clear<D: Draw<T = T>>(&mut self, d: &D) {
        d.draw(self, 0, 0);
        self.fill(self.get(0, 0).clone());
    }

    /// `set`s a pixel directly in the surface
    pub fn set(&mut self, x: usize, y: usize, val: T) {
        self.buf[x + y * self.surface_size.0] = val
    }

    /// Takes a position on the canvas and calculates the position of the top-most corner of the rectangle that corresponds
    /// to that pixel on the surface
    pub fn canvas_to_surface(&self, x: i32, y: i32) -> (usize, usize) {
        let (x, y) = {
            #[cfg(not(feature = "wrap"))]
            {
                let x = {
                    if x <= 0 {
                        0.
                    } else {
                        x as f32
                    }
                };
                let y = {
                    if y <= 0 {
                        0.
                    } else {
                        y as f32
                    }
                };
                (x, y)
            }
            #[cfg(feature = "wrap")]
            {
                let x = modv2(x, self.canvas_size.0 as i32) as f32;
                let y = modv2(y, self.canvas_size.1 as i32) as f32;
                (x, y)
            }
        };
        (
            round(x * self.ratio.0) as usize,
            round(y * self.ratio.1) as usize,
        )
    }

    /// Returns a reference to the value in the desired location on the canvas.
    pub fn get(&self, x: i32, y: i32) -> &T {
        let (x, y) = self.canvas_to_surface(x, y);
        #[cfg(not(feature = "wrap"))]
        {
            if x < self.surface_size.0 && y < self.surface_size.1 {
                &self.buf[x + y * self.surface_size.0]
            } else {
                self.buf.last().expect("Buffer in Canvas is empty")
            }
        }
        #[cfg(feature = "wrap")]
        {
            &self.buf[x + y * self.surface_size.0]
        }
    }

    /// Returns a mutable reference to the value in the desired location on the canvas.
    pub fn get_mut(&mut self, x: i32, y: i32) -> &mut T {
        let (x, y) = self.canvas_to_surface(x, y);
        #[cfg(not(feature = "wrap"))]
        {
            if x < self.surface_size.0 && y < self.surface_size.1 {
                &mut self.buf[x + y * self.surface_size.0]
            } else {
                self.buf.last_mut().expect("Buffer in Canvas is empty")
            }
        }
        #[cfg(feature = "wrap")]
        {
            &mut self.buf[x + y * self.surface_size.0]
        }
    }

    /// Returns a reference to the value in the desired location on the surface.
    pub fn get_surface(&self, x: usize, y: usize) -> &T {
        &self.buf[x + y * self.surface_size.0]
    }

    /// Returns a mutable reference to the value in the desired location on the surface.
    pub fn get_surface_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.buf[x + y * self.surface_size.0]
    }

    /// Draws any `Drawable` object
    pub fn draw<D: Draw<T = T>>(&mut self, x: i32, y: i32, d: &D) {
        d.draw(self, x, y);
    }

    /// 'Put's a value to the specified position on the canvas
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

    /// `draw`s a rectangle to the specified position on the canvas
    pub fn rect<D: Draw<T = T>>(&mut self, x: i32, y: i32, w: usize, h: usize, d: &D) {
        self.draw(x, y, &Rect { w, h, d });
    }

    /// `draw`s a line on the canvas using Bresenham's algorithm (no anti aliasing).
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

/// Primitive `draw`able shape that holds a single value that can be `put` onto the `Canvas`.
pub struct Pixel<T: Clone>(T);
impl<P: Clone> Draw for Pixel<P> {
    type T = P;

    fn draw(&self, canvas: &mut Canvas<'_, Self::T>, x: i32, y: i32) {
        canvas.put(x, y, self.0.clone());
    }
}

/// Primitive `draw`able shape that can be used to `draw` a rectangle on the `Canvas`.
pub struct Rect<'a, D: Draw> {
    pub w: usize,
    pub h: usize,
    pub d: &'a D,
}

impl<P: Clone, D: Draw<T = P>> Draw for Rect<'_, D> {
    type T = P;

    fn draw(&self, canvas: &mut Canvas<'_, Self::T>, x: i32, y: i32) {
        let mut y_counter = y;
        for _ in 0..self.h {
            let mut x_counter = x;
            for _ in 0..self.w {
                self.d.draw(canvas, x_counter, y_counter);
                x_counter += 1;
            }
            y_counter += 1;
        }
    }
}

/// Primitive `draw`able shape that can be used to `draw` a line on the `Canvas`.
pub struct Line<'a, D: Draw> {
    pub end_x: i32,
    pub end_y: i32,
    pub d: &'a D,
}

impl<P: Clone, D: Draw<T = P>> Draw for Line<'_, D> {
    type T = P;

    fn draw(&self, canvas: &mut Canvas<'_, Self::T>, x: i32, y: i32) {
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
            self.d.draw(canvas, x0, y0);
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
}

#[derive(Clone)]
/// The Rgb variant is converted to the 00000000RRRRRRRRGGGGGGGGBBBBBBBB format when .draw() is called, for custom pixel formats, use the Pixel variant.
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
    fn draw(&self, canvas: &mut Canvas<'_, u32>, x: i32, y: i32) {
        canvas.put(
            x,
            y,
            match self {
                Self::Rgb(red, green, blue) => {
                    ((*red as u32) << 16) | ((*green as u32) << 8) | (*blue as u32)
                }
                RGBu32::Pixel(p) => *p,
            },
        );
    }
}
