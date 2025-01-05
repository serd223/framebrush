use framebrush::{Canvas, Draw};

const BUF_WIDTH: usize = 32;
const BUF_HEIGHT: usize = 32;

enum Char {
    HashTag,
    AtSign,
}

impl Draw for Char {
    type T = char;

    fn draw(&self, canvas: &mut Canvas<'_, Self::T>, x: i32, y: i32) {
        canvas.put(
            x,
            y,
            match self {
                Char::HashTag => '#',
                Char::AtSign => '@',
            },
        );
    }
}

fn main() {
    let mut buf = [' '; BUF_HEIGHT * BUF_WIDTH];

    let mut canvas = Canvas::new(&mut buf, (BUF_WIDTH, BUF_HEIGHT), (BUF_WIDTH, BUF_HEIGHT));

    canvas.rect(0, 0, 5, 5, &Char::AtSign);
    canvas.line(0, 31, 25, 16, &Char::HashTag);

    for y in 0..BUF_HEIGHT {
        let stripe = &buf[(y * BUF_WIDTH)..((y + 1) * BUF_WIDTH)];
        for c in stripe {
            print!("{}", c);
        }
        println!();
    }
}
