extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
mod bar;
mod ball;
use bar::Bar;
use ball::Ball;

const MOVESPEED: i32 = 15;
const BLACK: Color = Color::RGB(0, 0, 0);
const GREEN: Color = Color::RGB(161, 209, 174);

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("test-sdl2", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(BLACK);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut bar1 = Bar::new(20, 20);
    let mut bar2 = Bar::new(740, 20);
    let mut ball = Ball::new(400, 300);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => if !is_at_bottom_border(bar1) {bar1.mv(MOVESPEED)} else {},
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => if !is_at_top_border(bar1) {bar1.mv(-MOVESPEED)} else {},
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => if !is_at_bottom_border(bar2) {bar2.mv(MOVESPEED)} else {},
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => if !is_at_top_border(bar2) {bar2.mv(-MOVESPEED)} else {},
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        ball.update_pos();

        canvas.set_draw_color(BLACK);
        canvas.clear();
        canvas.set_draw_color(GREEN);

        let rect = bar1.to_rect();
        let rect2 = bar2.to_rect();
        let ball_rect = ball.to_rect();

        canvas.draw_rect(rect).unwrap();
        canvas.fill_rect(rect).unwrap();
        canvas.draw_rect(rect2).unwrap();
        canvas.fill_rect(rect2).unwrap();
        canvas.draw_rect(ball_rect).unwrap();
        canvas.fill_rect(ball_rect).unwrap();
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1000000000u32 / 60));
    }

    // Helpers
    fn is_at_top_border(bar: Bar) -> bool{
        bar.pos_y <= 20
    }

    // Problem: we are considering a hardcoded 160 height for the bars
    fn is_at_bottom_border(bar: Bar) -> bool{
        bar.pos_y >= 420
    }
}
