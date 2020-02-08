extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;
mod bar;

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
    let mut bar1 = bar::Bar::new(20, 20);
    let mut bar2 = bar::Bar::new(740, 20);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => bar1.mv(5),
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => bar1.mv(-5),
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
        let rect = Rect::new(bar1.pos_x, bar1.pos_y, bar1.width, bar1.heigth);
        canvas.draw_rect(rect).unwrap();
        canvas.fill_rect(rect).unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1000000000u32 / 60));
    }
}
