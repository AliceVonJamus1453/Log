use std::{thread};
use std::time::{Duration, Instant};
use sdl3::event::Event;
use sdl3::pixels::Color;
use sdl3::render::{FPoint, WindowCanvas};
use glam::Vec2;

fn main() {
    let clear_color = Color::RGBA(0, 0, 0, 255);
    let herat_color = Color::RGBA(255, 0, 0, 255);

    let sdl3 = sdl3::init().unwrap();
    let video = sdl3.video().unwrap();
    let window = video.window("爱心", 800, 600)
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas();

    let mut event_pump = sdl3.event_pump().unwrap();
    let time_check = Duration::from_millis(1000 / 60);
    let mut i = 0.000;
    let mut adjust = 0.01;
    'Running: loop {
        let clock = Instant::now();
        canvas.set_draw_color(clear_color);
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {

                Event::Quit {..} => {
                    break 'Running
                }

                _ => {}
            }
        }

        i = 0.000;
        canvas.set_draw_color(herat_color);
        while i <= 6.300 {
            let location = herat_location(i)
                .map(|x| -(f32::sin(adjust) * x * 5.0) + 200.0);

            canvas.draw_point(FPoint::new(location.x, location.y)).unwrap();
            i += 0.001;
            adjust += 0.01;
        }
        canvas.present();

        let duration = clock.duration_since(clock);
        if duration < time_check {
            thread::sleep(time_check - duration);
        }
    }
}

fn herat_location(phase: f32) -> Vec2 {
    Vec2::new(
        16.0 * f32::sin(phase) * f32::sin(phase) * f32::sin(phase),
        13.0 * f32::cos(phase) - 5.0 * f32::cos(2.0 * phase) - 2.0 * f32::cos(3.0 * phase) - f32::cos(4.0 * phase)
    )
}

