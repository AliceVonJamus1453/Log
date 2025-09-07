use std::{thread};
use std::time::{Duration, Instant};
use sdl3::event::Event;
use sdl3::pixels::Color;
use sdl3::render::FRect;
use sdl3::render::BlendMode;
use glam::f32::Vec2;
use rand::{random, random_range};
use sdl3::keyboard::Keycode;

fn main() {
    let (window_x, window_y) = (800, 600);
    let particle_num = 10000;
    let clear_color = Color::RGBA(0, 0, 0, 0);
    let mut hole_m:f32 = 0.0;

    let sdl3 = sdl3::init().unwrap();
    let video_system = sdl3.video().unwrap();
    let window = video_system.window("指尖重力", window_x, window_y)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas();
    canvas.set_blend_mode(BlendMode::Blend);

    let window_x = window_x as f32;
    let window_y = window_y as f32;

    let mut all:Vec<Particle> = Vec::with_capacity(particle_num);
    for _ in 0..all.capacity() {
        all.push(
            Particle{
                f_rect: FRect::new(
                    random_range(-10.0 ..= window_x + 10.0),
                    random_range(-10.0 ..= window_y + 10.0),
                    4.0,
                    4.0
                ),
                color: Color::RGBA(
                    random(),
                    random(),
                    random(),
                    255
                ),
                weight: random_range(1.0 ..= 5.0)
            }
        )
    }

    let mut mouse = Vec2::new(0.0,0.0);
    let mut event_pump = sdl3.event_pump().unwrap();
    let time_check = Duration::from_millis(1000 / 120);
    'Running: loop {
        let clock = Instant::now();
        canvas.set_draw_color(clear_color);
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {

                Event::MouseMotion {
                    x,
                    y,
                    ..
                } => {
                    mouse.x = x;
                    mouse.y = y;
                }

                Event::KeyDown {
                    keycode,
                    ..
                } => {
                    match keycode.unwrap() {

                        Keycode::J => hole_m -= 1000.0,
                        Keycode::K => hole_m += 1000.0,
                        Keycode::Up => hole_m += 200.0,
                        Keycode::Down => hole_m -= 200.0,
                        Keycode::Escape => hole_m = 0.0,

                        _ => {}
                    }
                    println!("当前鼠标的重量是{}", hole_m)
                }

                Event::Quit {..} => {
                    break 'Running
                }

                _ => {}
            }
        }

        for particle in all.iter_mut() {
            canvas.set_draw_color(particle.color);
            canvas.draw_rect(particle.f_rect).unwrap();

            let this = Vec2::new(particle.x(), particle.y());
            let force = gravity(this, mouse, particle.weight, hole_m, 10.0);
            particle
                .normal()
                .move_to(mouse, force)
                .check((mouse.x,mouse.y),(window_x,window_y));
        }
        canvas.present();

        let duration = clock.duration_since(clock);
        if duration < time_check {
            thread::sleep(time_check - duration);
        }
    }
}

struct Particle {
    f_rect: FRect,
    color: Color,
    weight: f32
}

impl Particle {
    fn x(&self) -> f32 {
        self.f_rect.x
    }

    fn y(&self) -> f32 {
        self.f_rect.y
    }

    fn location(&mut self) -> Vec2 {
        Vec2::new(self.x(), self.y())
    }

    fn move_to(&mut self, target: Vec2, force: f32) -> &mut Self {
        let delta = self.location().move_towards(target, force);
        self.f_rect.set_x(delta.x);
        self.f_rect.set_y(delta.y);
        self
    }

    fn check(&mut self, (x,y):(f32,f32), (window_x,window_y):(f32,f32)) -> &mut Self {
        if self.x() >= window_x ||
            self.x() <= 0.0 ||
            self.y() >= window_y ||
            self.y() <= 0.0 ||
            (self.x() == x && self.y() == y )
        {
            self.f_rect.set_x(random_range(0.0 .. window_x));
            self.f_rect.set_y(random_range(0.0 .. window_y));
        }
        self
    }

    fn normal(&mut self) -> &mut Self {
        self.move_to(
            Vec2::new(
                random_range(self.x() - 5.0 ..  self.x() + 5.0),
                random_range(self.y() - 5.0 .. self.y() + 5.0)
            ),
            random_range(0.0..1.0)
        );
        self
    }
}

fn gravity(this:Vec2, other:Vec2, this_m:f32, other_m:f32, cor:f32) -> f32{
    cor * this_m * other_m / this.distance_squared(other)
}

