use sdl2::rect::Rect;

#[derive(Copy, Clone)]
pub struct Ball {
    pub pos_x: i32,
    pub pos_y: i32,
    pub heigth: u32,
    pub width: u32,
    pub dx: i32,
    pub dy: i32,
}

impl Ball {
    pub fn new(pos_x: i32, pos_y: i32) -> Ball {
        Ball {
            pos_x: pos_x,
            pos_y: pos_y,
            heigth: 20,
            width: 20,
            dx: 5,
            dy: 3,
        }
    }

    pub fn update_pos(&mut self) -> () {
        self.pos_x += self.dx;
        self.pos_y += self.dy;
    }

    pub fn to_rect(&self) -> Rect {
        Rect::new(self.pos_x, self.pos_y, self.width, self.heigth)
    }
}
