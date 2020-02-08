extern crate sdl2;

mod ball;
mod bar;
mod momentum;

use ball::Ball;
use bar::Bar;
use momentum::Momentum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

const MOVESPEED: i32 = 15;
const CONTACT_THRESHOULD: u32 = 2;
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
                } => {
                    if !is_at_bottom_border(bar1) {
                        bar1.mv(MOVESPEED)
                    } else {
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    if !is_at_top_border(bar1) {
                        bar1.mv(-MOVESPEED)
                    } else {
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if !is_at_bottom_border(bar2) {
                        bar2.mv(MOVESPEED)
                    } else {
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    if !is_at_top_border(bar2) {
                        bar2.mv(-MOVESPEED)
                    } else {
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

        ball.update_pos();
        ball.update_momentum(get_new_momentum(ball, bar1, bar2));

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
    fn is_at_top_border(bar: Bar) -> bool {
        bar.pos_y <= 20
    }

    // Problem: we are considering a hardcoded 160 height for the bars
    fn is_at_bottom_border(bar: Bar) -> bool {
        bar.pos_y >= 420
    }

    fn get_new_momentum(ball: Ball, bar1: Bar, bar2: Bar) -> Momentum {
        let mut new_momentum = ball.momentum;
        if ball.pos_y <= 20 {
            new_momentum = Momentum{dy: 2, dx: ball.momentum.dx}
        } else if ball.pos_y >= 560 {
            new_momentum = Momentum{dy: -2, dx: ball.momentum.dx}
        } else if ball.pos_x <= (20 + bar1.width + CONTACT_THRESHOULD) {
            if test_collision(ball, bar1) {
                new_momentum = calc_new_momentum(ball, bar1);
                println!("Colidiu, porra!");
            }
            println!("Área de colisão da bar1!");
        } else if ball.pos_x >= (780 - bar2.width - CONTACT_THRESHOULD - ball.width) {
            if test_collision(ball, bar2) {
                new_momentum = calc_new_momentum(ball, bar2);
                println!("Colidiu, porra!");
                println!("{}, {}, {}, {}", 800, bar2.width, CONTACT_THRESHOULD, 800 - bar2.width - CONTACT_THRESHOULD)
            }
            println!(
                "Área de colisão da bar2! ball_y: {}, bar_y: {}",
                ball.pos_y, bar2.pos_y
            );
        } else {
        }

        new_momentum
    }

    fn test_collision(ball: Ball, bar: Bar) -> bool {
        ball.pos_y > bar.pos_y && ball.pos_y < bar.pos_y + bar.heigth
    }

    fn calc_new_momentum(ball: Ball, colliding_bar: Bar) -> Momentum {
        let bar_fragment_size = colliding_bar.heigth / 4;
        let bar_fragment_touched = (ball.pos_y - colliding_bar.pos_y) / bar_fragment_size;

        match (ball.pos_x, bar_fragment_touched) {
            (x, 0) if x < 400 => Momentum { dx: 1, dy: -2 },
            (x, 1) if x < 400 => Momentum { dx: 2, dy: -1 },
            (x, 2) if x < 400 => Momentum { dx: 2, dy: 1 },
            (x, 3) if x < 400 => Momentum { dx: 1, dy: 2 },
            (x, 4) if x < 400 => Momentum { dx: 1, dy: 2 },
            (x, 0) if x > 400 => Momentum { dx: -1, dy: -2 },
            (x, 1) if x > 400 => Momentum { dx: -2, dy: -1 },
            (x, 2) if x > 400 => Momentum { dx: -2, dy: 1 },
            (x, 3) if x > 400 => Momentum { dx: -1, dy: 2 },
            (x, 4) if x > 400 => Momentum { dx: -1, dy: 2 },
            _ => Momentum { dx: 0, dy: 0 },
        }
    }
}
