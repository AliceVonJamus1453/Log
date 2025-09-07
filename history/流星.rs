use sdl3::pixels::{Color};
use sdl3::render::{BlendMode, WindowCanvas};
use sdl3::event::{Event};
use sdl3::rect::Rect;
use sdl3::keyboard::Keycode;
use std::time::{Duration, Instant};
use std::{thread};
use rand::{random_range};

fn main() {
    let mut rain_color = Color::RGBA(195, 192, 15, 120);
    let window_size: (u32, u32) = (1920, 1080);
    let rain_num:usize = 200;
    let sora_num:usize = 10000;

    let sdl3_server = sdl3::init().unwrap();
    let video_sub_sys = sdl3_server.video().unwrap();

    let window = video_sub_sys.window("流星", window_size.0, window_size.1)
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.clone().into_canvas();
    canvas.set_blend_mode(BlendMode::Blend);

    let mut rain:Vec<Rain> = Vec::with_capacity(rain_num);
    for _ in 0..rain.capacity() {
        rain.push(
            Rain{
                rect: Rect::new(
                    random_range(0 .. window_size.0 as i32),
                    random_range(0 .. window_size.1 as i32),
                    8, 8
                ),
                x_speed: random_range(1..=5),
                y_speed: random_range(1..=5)
            }
        )
    }

    let mut sora:Vec<Rect> = Vec::with_capacity(sora_num);
    for _ in 0.. sora.capacity() {
        sora.push(Rect::new(
            random_range(0..=window_size.0) as i32,
            random_range(0..=window_size.1) as i32,
            5,
            5)
        )
    }

    let mut event_pump = sdl3_server.event_pump().unwrap();
    let time_check = Duration::from_millis(1000 / 120);
    'Running: loop {
        let clock = Instant::now();

        canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        canvas.clear();
        for i in 0..sora.capacity() {
            if i as i32 % 10 == 0 {
                canvas.set_draw_color(Color::RGBA(255,255,255, random_range(0..=80)));
            }
            else {
                canvas.set_draw_color(Color::RGBA(255,255,255, random_range(0..=20)));
            }
            canvas.fill_rect(sora[i]).unwrap();
        }

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

        canvas.set_draw_color(rain_color);
        for rain in rain.iter_mut() {
            canvas.fill_rect(rain.rect).unwrap();
            rain.run().trailing(20, &mut canvas, &mut rain_color).check(window.size());
        }
        canvas.present();

        let duration = clock.duration_since(clock);
        if duration < time_check {
            thread::sleep(time_check - duration);
        }
    }
}

struct Rain {
    rect: Rect,
    x_speed: i32,
    y_speed: i32
}

impl Rain {
    fn run(&mut self) -> &mut Self {
        self.rect.x += self.x_speed;
        self.rect.y += self.y_speed;
        self
    }

    fn check(&mut self, (width, height): (u32, u32)) -> &mut Self {
        self.rect.x %= width as i32 + 30;
        self.rect.y %= height as i32 + 30;
        self
    }

    fn trailing(&mut self, long: u8, canvas: &mut WindowCanvas, color: &mut Color) -> &mut Self {
        if long ==0 {
            return self
        }

        let mut max: u8 = 255;
        let mut rect = self.rect.clone();
        while max > long {
            max -= long;
            rect.x -= self.x_speed;
            rect.y -= self.y_speed;
            color.a = max;
            canvas.set_draw_color(*color);
            canvas.fill_rect(rect).unwrap()
        }
        self
    }
}

