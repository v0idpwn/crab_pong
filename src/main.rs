extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("test-sdl2", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let position_x = 20;
    let mut position_y = 20;
    let player_width = 40;
    let player_heigth = 160;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    position_y = if is_colliding_with_bottom_border(position_y) {
                        position_y
                    } else {
                        position_y + 5
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    position_y = if is_colliding_with_top_border(position_y) {
                        position_y
                    } else {
                        position_y - 5
                    }
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(161, 209, 174));
        let rect = Rect::new(position_x, position_y, player_width, player_heigth);
        canvas.draw_rect(rect).unwrap();
        canvas.fill_rect(rect).unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1000000000u32 / 60));
    }
}

fn is_colliding_with_top_border(position_y: i32) -> bool {
    position_y <= 20
}

fn is_colliding_with_bottom_border(position_y: i32) -> bool {
    position_y >= (600 - 160 - 20)
}
