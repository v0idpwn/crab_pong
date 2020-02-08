use sdl2::rect::Rect;

#[derive(Copy, Clone)]
pub struct Bar {
    pub pos_x: u32,
    pub pos_y: u32,
    pub heigth: u32,
    pub width: u32,
}

impl Bar {
    pub fn new(pos_x: u32, pos_y: u32) -> Bar {
        Bar {
            pos_x: pos_x,
            pos_y: pos_y,
            heigth: 160,
            width: 40,
        }
    }

    pub fn mv(&mut self, qty: i32) -> () {
        if qty > 0 {
            self.pos_y += qty as u32;
        } else {
            self.pos_y -= qty.abs() as u32;
        }
    }

    pub fn to_rect(&self) -> Rect {
        Rect::new(
            self.pos_x as i32,
            self.pos_y as i32,
            self.width,
            self.heigth,
        )
    }
}
