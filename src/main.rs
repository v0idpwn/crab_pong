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
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const BORDER: u32 = 20;
const BAR_HEIGHT: u32 = 160;
const BAR_WIDTH: u32 = 40;
const BALL_HEIGHT: u32 = 20;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("test-sdl2", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(BLACK);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut bar1 = Bar::new(BORDER, BORDER);
    let mut bar2 = Bar::new(WIDTH - BAR_WIDTH - BORDER, BORDER);
    let mut ball = Ball::new(WIDTH / 2, HEIGHT / 2);
    let mut score1 = 0 as u32;
    let mut score2 = 0 as u32;

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

        // Game routines
        match goal_check(ball) {
            (true, 1) => {
                score1 += 1;
                ball.set_position(WIDTH / 2, HEIGHT / 2);
                println!("{} x {}", score1, score2);
            }
            (true, 2) => {
                score2 += 1;
                ball.set_position(WIDTH / 2, HEIGHT / 2);
                println!("{} x {}", score1, score2);
            }
            _ => {}
        }

        ball.update_pos();

        ball.update_momentum(get_new_momentum(ball, bar1, bar2));

        // Drawing on screen
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
        bar.pos_y <= BORDER
    }

    fn is_at_bottom_border(bar: Bar) -> bool {
        bar.pos_y >= HEIGHT - BAR_HEIGHT - BORDER
    }

    fn get_new_momentum(ball: Ball, bar1: Bar, bar2: Bar) -> Momentum {
        let mut new_momentum = ball.momentum;

        if ball.pos_y <= BORDER {
            new_momentum = Momentum {
                dy: -ball.momentum.dy,
                dx: ball.momentum.dx,
            }
        } else if ball.pos_y >= (HEIGHT - BORDER - BALL_HEIGHT) {
            new_momentum = Momentum {
                dy: -ball.momentum.dy,
                dx: ball.momentum.dx,
            }
        } else if ball.pos_x <= (BORDER + bar1.width + CONTACT_THRESHOULD) {
            if test_collision(ball, bar1) {
                new_momentum = calc_new_momentum(ball, bar1);
            }
        } else if ball.pos_x >= (WIDTH - BORDER - bar2.width - CONTACT_THRESHOULD - ball.width) {
            if test_collision(ball, bar2) {
                new_momentum = calc_new_momentum(ball, bar2);
            }
        } else {
        }

        new_momentum
    }

    fn test_collision(ball: Ball, bar: Bar) -> bool {
        ball.pos_y > bar.pos_y && ball.pos_y < bar.pos_y + bar.heigth
    }

    fn calc_new_momentum(ball: Ball, colliding_bar: Bar) -> Momentum {
        let bar_fragment_size = colliding_bar.heigth / 5;
        let bar_fragment_touched = (ball.pos_y - colliding_bar.pos_y) / bar_fragment_size;

        match (ball.pos_x, bar_fragment_touched) {
            (x, 0) if x < WIDTH / 2 => Momentum { dx: 1, dy: -3 },
            (x, 1) if x < WIDTH / 2 => Momentum { dx: 2, dy: -2 },
            (x, 2) if x < WIDTH / 2 => Momentum { dx: 3, dy: 1 },
            (x, 3) if x < WIDTH / 2 => Momentum { dx: 2, dy: 2 },
            (x, 4) if x < WIDTH / 2 => Momentum { dx: 1, dy: 3 },
            (x, 0) if x > WIDTH / 2 => Momentum { dx: -1, dy: -3 },
            (x, 1) if x > WIDTH / 2 => Momentum { dx: -2, dy: -2 },
            (x, 2) if x > WIDTH / 2 => Momentum { dx: -3, dy: 1 },
            (x, 3) if x > WIDTH / 2 => Momentum { dx: -2, dy: 2 },
            (x, 4) if x > WIDTH / 2 => Momentum { dx: -1, dy: 3 },
            _ => Momentum { dx: 0, dy: 0 },
        }
    }

    fn goal_check(ball: Ball) -> (bool, u32) {
        let mut goal_happened = false;
        let mut goal_for = 0;

        if ball.pos_x > WIDTH - (2 * BORDER) {
            goal_happened = true;
            goal_for = 1;
        } else if ball.pos_x < (2 * BORDER) {
            goal_happened = true;
            goal_for = 2;
        }

        (goal_happened, goal_for)
    }
}
