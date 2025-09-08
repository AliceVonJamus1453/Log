mod base;
use crate::base::anime::Anime;
use crate::base::charactar::Charactar;

use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};
use sdl3::event::Event;
use sdl3::pixels::{Color, PixelFormat, PixelFormatEnum};
use sdl3::rect::Rect;
use sdl3::render::{BlendMode, TextureAccess, TextureCreator, Texture, Canvas, WindowCanvas};
use sdl3::keyboard::Keycode;
use sdl3::video::WindowContext;

fn main() {
    let (window_x,window_y):(u32,u32) = (800,600);
    let player_speed = 1;

    let mut up_move = false;
    let mut down_move = false;
    let mut left_move = false;
    let mut right_move = false;

    let sdl3 = sdl3::init().unwrap();
    let video_subsystem = sdl3.video().unwrap();
    let window = video_subsystem.window("读取图片测试", window_x,window_y)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas();
    canvas.set_blend_mode(BlendMode::Blend);
    let creator = canvas.texture_creator();
    
    let anime = Anime::from(
        Path::new("./art/charactar/alice/stand"),
        16,
        "png",
        &creator,
        TextureAccess::Static
    );
    let entity = Rect::new(0, 0, anime.width(), anime.height());
    let mut player = Charactar::new(entity,&anime);

    const TIME_CHECK: Duration = Duration::from_millis(1000 / 120);
    let mut fps:u32= 0;
    let mut event_pump = sdl3.event_pump().unwrap();
    'Running: loop {
        let loop_start = Instant::now();
        fps += 1;
        fps %= 10;

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {

                Event::KeyDown {
                    keycode,
                    ..
                } => {
                    match keycode.unwrap() {

                        Keycode::Left => left_move = true,
                        Keycode::Right => right_move = true,
                        Keycode::Up => up_move = true,
                        Keycode::Down => down_move = true,

                        _ => {}
                    }
                }

                Event::KeyUp {
                    keycode,
                    ..
                } => {
                    match keycode.unwrap() {

                        Keycode::Left => left_move = false,
                        Keycode::Right => right_move = false,
                        Keycode::Up => up_move = false,
                        Keycode::Down => down_move = false,

                        Keycode::Escape => break 'Running,
                        _ => {}
                    }

                }

                Event::Quit {..} => break 'Running,

                _ => {}
            }
        }

        if left_move {
            player.set_x(player.x() - player_speed);
        }
        if right_move {
            player.set_x(player.y() - player_speed);
        }
        if up_move {
            player.set_y(player.x() - player_speed);
        }
        if down_move {
            player.set_y(player.x() + player_speed);
        }

        player.normal(&mut canvas,fps);
        canvas.present();

        let loop_over = loop_start.duration_since(loop_start);
        if loop_over < TIME_CHECK {
            thread::sleep(TIME_CHECK- loop_over)
        }
    }
}