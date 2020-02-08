pub struct Bar {
    pos_x: i32,
    pos_y: i32,
    heigth: i32,
    width: i32,
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
}
