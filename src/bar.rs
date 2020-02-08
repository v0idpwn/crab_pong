use sdl2::rect::Rect;

#[derive(Copy, Clone)]
pub struct Bar {
    pub pos_x: i32,
    pub pos_y: i32,
    pub heigth: u32,
    pub width: u32,
}

impl Bar {
    pub fn new(pos_x: i32, pos_y: i32) -> Bar {
        Bar {
            pos_x: pos_x,
            pos_y: pos_y,
            heigth: 160,
            width: 40
        }
    }

    pub fn mv(&mut self, qty: i32) -> () {
        self.pos_y += qty;
    }

    pub fn to_rect(&self) -> Rect {
        Rect::new(self.pos_x, self.pos_y, self.width, self.heigth)
    }
}
