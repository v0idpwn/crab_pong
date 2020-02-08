extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;
mod bar;
use bar::Bar;

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
    let mut bar1 = Bar::new(20, 20);
    let mut bar2 = Bar::new(740, 20);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => if !is_at_bottom_border(bar1) {bar1.mv(5)} else {},
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => if !is_at_top_border(bar1) {bar1.mv(-5)} else {},
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => if !is_at_bottom_border(bar2) {bar2.mv(5)} else {},
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => if !is_at_top_border(bar2) {bar2.mv(-5)} else {},
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
        let rect2 = Rect::new(bar2.pos_x, bar2.pos_y, bar2.width, bar2.heigth);

        canvas.draw_rect(rect).unwrap();
        canvas.fill_rect(rect).unwrap();
        canvas.draw_rect(rect2).unwrap();
        canvas.fill_rect(rect2).unwrap();
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1000000000u32 / 60));
    }

    fn is_at_top_border(bar: Bar) -> bool{
        bar.pos_y <= 20
    }

    // Problem: we are considering a hardcoded 160 height for the bars
    fn is_at_bottom_border(bar: Bar) -> bool{
        bar.pos_y >= 420
    }
}
