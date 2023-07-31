use framebrush::{Canvas, Color};

const BUF_WIDTH: usize = 32;
const BUF_HEIGHT: usize = 32;

enum CharColor {
    HashTag,
    AtSign,
}

impl Color<char> for CharColor {
    fn pixel(&self, _buf: &mut [char], _idx: usize) -> char {
        match self {
            Self::HashTag => '#',
            Self::AtSign => '@',
        }
    }
}

fn main() {
    let mut buf = [' '; BUF_HEIGHT * BUF_WIDTH];

    let mut canvas = Canvas::new(&mut buf, (BUF_WIDTH, BUF_HEIGHT), (BUF_WIDTH, BUF_HEIGHT));

    canvas.put_rect(0, 0, 5, 5, &CharColor::AtSign);
    canvas.put_line(0, 31, 25, 16, &CharColor::HashTag);

    for y in 0..BUF_HEIGHT {
        let stripe = &buf[(y * BUF_WIDTH)..((y + 1) * BUF_WIDTH)];
        for c in stripe {
            print!("{}", c);
        }
        println!();
    }
}
