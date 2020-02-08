use sdl2::rect::Rect;

use crate::momentum::Momentum;

#[derive(Copy, Clone)]
pub struct Ball {
    pub pos_x: u32,
    pub pos_y: u32,
    pub heigth: u32,
    pub width: u32,
    pub momentum: Momentum
}

impl Ball {
    pub fn new(pos_x: u32, pos_y: u32) -> Ball {
        Ball {
            pos_x: pos_x,
            pos_y: pos_y,
            heigth: 20,
            width: 20,
            momentum: Momentum {dx: 2, dy: 1}
        }
    }

    // This is plain stupid
    pub fn update_pos(&mut self) -> () {
        if self.momentum.dx > 0 {
            self.pos_x += self.momentum.dx as u32;
        } else {
            self.pos_x -= self.momentum.dx.abs() as u32;
        }

        if self.momentum.dy > 0 {
            self.pos_y += self.momentum.dy as u32;
        } else {
            self.pos_y -= self.momentum.dy.abs() as u32;
        }
    }

    pub fn set_position(&mut self, x: u32, y: u32) -> () {
        self.pos_x = x;
        self.pos_y = y;
    }

    pub fn update_momentum(&mut self, m: Momentum) -> () {
        self.momentum = m;
    }

    pub fn to_rect(&self) -> Rect {
        Rect::new(self.pos_x as i32, self.pos_y as i32, self.width, self.heigth)
    }
}
