use sdl3::pixels::{Color};
use sdl3::render::{BlendMode};
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::rect::Rect;

use std::time::{Duration, Instant};
use std::{thread};

fn main() {
    let sdl3_server = sdl3::init().unwrap();
    let video_sub_sys = sdl3_server.video().unwrap();

    let window = video_sub_sys.window("Text", 800, 600)
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();
    canvas.set_blend_mode(BlendMode::Blend);
    canvas.clear();

    let mut x = 400;
    let mut y = 300;

    let mut event_pump = sdl3_server.event_pump().unwrap();
    let time_check = Duration::from_millis(1000 / 60);
    let clear = Rect::new(0, 0, 800, 600);
    canvas.fill_rect(clear).unwrap();
    'Running: loop {
        let clock = Instant::now();

        canvas.set_draw_color(Color::RGBA(0, 0, 0, 20));
        canvas.fill_rect(clear).unwrap();

        for event in event_pump.poll_iter() {
            match event {

                Event::KeyDown {
                    keycode,
                    ..
                } => {
                    match keycode.unwrap() {

                        Keycode::Left => x -= 5,
                        Keycode::Right => x += 5,
                        Keycode::Up => y -= 5,
                        Keycode::Down => y += 5,

                        _ => {}
                    }
                }

                Event::Quit {..} => {
                    break 'Running
                }

                _ => {}
            }
        }

        let rect = Rect::new(x, y, 20, 40);
        canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
        canvas.fill_rect(rect).unwrap();
        canvas.present();

        let duration = clock.duration_since(clock);
        if duration < time_check {
            thread::sleep(time_check - duration);
        }
    }
}