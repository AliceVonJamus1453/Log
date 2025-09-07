use std::{f32, thread};
use std::time::{Duration, Instant};
use sdl3::event::Event;
use sdl3::pixels::{Color, FColor};
use sdl3::render::{BlendMode, Canvas, FPoint, WindowCanvas};
use glam::{Vec2};
use rand::random_range;
use sdl3::keyboard::Keycode;

fn main() {
    let heart_num = 20;
    let window_x = 1920;
    let window_y = 1080;

    let sdl3 = sdl3::init().unwrap();
    let video = sdl3.video().unwrap();
    let window = video.window("爱心", window_x, window_y)
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas();
    canvas.set_blend_mode(BlendMode::Blend);
    let window_x = window_x as f32;
    let window_y = window_y as f32;

    let mut size = window_y / 2.0;
    let mut out = FPoint::new(0.0, 0.0);
    let mut side = FPoint::new(0.0, 0.0);
    let mut vec2 = Vec2::new(0.0, 0.0);

    let mut all:Vec<Heart> = Vec::with_capacity(heart_num);
    for _ in 0..all.capacity() {
        all.push(Heart::new(
            (random_range(0.0 .. window_x),window_y + size),
            random_range(-5.0..=-1.5),
            random_range(0.0..=10.0),
        ))
    }

    let mut event_pump = sdl3.event_pump().unwrap();
    let time_check = Duration::from_millis(1000 / 120);
    'Running: loop {
        let clock = Instant::now();
        canvas.set_draw_color(FColor::RGBA(0.0, 0.0, 0.0, 0.0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {

                Event::KeyDown {
                    keycode,
                    ..
                } => {
                    match keycode.unwrap() {
                        Keycode::Escape => break 'Running,

                        _ => {}
                    }
                }

                Event::Quit {..} => {
                    break 'Running
                }

                _ => {}
            }
        }

        for heart in all.iter_mut(){
            heart.run()
                .draw_heart(
                    &mut canvas,
                    &mut size,
                    &mut out,
                    &mut side,
                    &mut vec2
                )
                .check(window_y, size);
            out.x = 0.0;out.y = 0.0;
            vec2.x = 0.0;vec2.y = 0.0;
        }
        canvas.present();

        let duration = clock.duration_since(clock);
        if duration < time_check {
            thread::sleep(time_check - duration);
        }
    }
}

struct Heart {
    location: (f32,f32),
    speed: f32,
    adjust: f32,
}

impl Heart {
    fn new(location:(f32,f32), speed:f32, adjust:f32) -> Self {
        Heart {
            location,
            speed,
            adjust,
        }
    }

    fn heart_location(phase: f32) -> Vec2 {
        Vec2::new(
            16.0 * f32::sin(phase) * f32::sin(phase) * f32::sin(phase),
            13.0 * f32::cos(phase) - 5.0 * f32::cos(2.0 * phase) - 2.0 * f32::cos(3.0 * phase) - f32::cos(4.0 * phase)
        )
    }

    fn run(&mut self) -> &mut Self {
        self.location.0 +=  f32::sin(self.adjust) / 0.5;
        self.location.1 += self.speed;
        self
    }

    fn draw_heart(
        &mut self,
        canvas:&mut WindowCanvas,
        size:&mut f32,
        out:&mut FPoint,
        side:&mut FPoint,
        vec2:&mut Vec2
    ) -> &mut Self {
        let mut i:f32 = 0.00;
        while i <= f32::consts::PI * 2.0 {
            *vec2 = Self::heart_location(i)
                .map(|x| -x * f32::abs(f32::sin(self.adjust * 6.5) + *size / 48.0))
                .map(|x| x );

            out.x = vec2.x + self.location.0;
            out.y = vec2.y + self.location.1;

            *vec2 = Self::heart_location(i)
                .map(|x| -x * f32::abs(f32::sin(self.adjust * 6.5) + *size / 60.0))
                .map(|x| x );

            side.x = vec2.x + self.location.0;
            side.y = vec2.y + self.location.1;

            canvas.set_draw_color(FColor::RGBA(f32::abs(f32::cos(self.adjust)), 0.0, f32::abs(f32::sin(self.adjust)),1.0));
            canvas.draw_line(*side,*out).unwrap();
            i += 0.005
        }
        self
    }

    fn check(
        &mut self,
        top:f32,
        size:f32
    ) -> &mut Self {
        if {
            Self::heart_location(f32::consts::PI)
                .map(|x| -x * f32::abs(f32::sin(self.adjust * 6.5) + size / 50.0))
                .map(|x| x + self.location.0).y + self.location.1 <= 0.0
        } {
            self.location.1 = size / 4.0 + top
        }
        self.adjust += 0.01;
        self
    }

}
