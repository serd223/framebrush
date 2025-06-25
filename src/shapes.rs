use crate::*;

/// Primitive `draw`able shape that can be used to `draw` a solid color rectangle on the `Canvas`.
pub struct SolidRect<'a, C: Color> {
    pub w: usize,
    pub h: usize,
    pub c: &'a C,
}

impl<T: Clone, C: Color<P = T>> Draw for SolidRect<'_, C> {
    type P = T;

    fn draw(&self, canvas: &mut Canvas<Self::P, &mut [Self::P]>, x: i32, y: i32) {
        let c = self.c.pixel();
        let (sx, sy) = canvas.canvas_to_surface(x, y);
        let (sw, sh) = canvas.canvas_to_surface(self.w as i32, self.h as i32);
        let start = sy * canvas.surface_size.0 + sx;
        let end = start + sw;
        let w = canvas.surface_size.0;
        for i in 0..sh {
            (canvas.buf[start + i * w..end + i * w]).fill(c.clone());
        }
    }
}

/// Primitive `draw`able shape that can be used to `draw` a rectangle on the `Canvas`.
pub struct Rect<'a, D: Draw> {
    pub w: usize,
    pub h: usize,
    pub d: &'a D,
}

impl<P: Clone, D: Draw<P = P>> Draw for Rect<'_, D> {
    type P = P;

    fn draw(&self, canvas: &mut Canvas<Self::P, &mut [Self::P]>, x: i32, y: i32) {
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

impl<P: Clone, D: Draw<P = P>> Draw for Line<'_, D> {
    type P = P;

    fn draw(&self, canvas: &mut Canvas<Self::P, &mut [Self::P]>, x: i32, y: i32) {
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
