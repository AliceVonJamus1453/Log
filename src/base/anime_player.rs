use std::time::{Duration, Instant};
use sdl3::rect::{Point, Rect};
use sdl3::render::{WindowCanvas, Texture, FPoint};
use crate::base::anime::Anime;
use crate::base::facing::Facing;

pub struct AnimePlayer<'a> {
    anime: &'a Anime<'a>,
    timer: Duration, //时间间隔
    clock: Instant, //计时器
    index: usize, //动画帧索引
    interval: Duration, //动画的图片切换间隔
}

//构造函数的方法集合
impl<'a> AnimePlayer<'a> {
    pub fn from(anime: &'a Anime, interval: Duration) -> AnimePlayer<'a> {
        AnimePlayer {
            anime,
            interval,
            index: 0,
            timer: Duration::default(),
            clock: Instant::now()
        }
    }
}

//动画播放的方法集合
impl<'a> AnimePlayer<'a> {
    pub fn normal_stop(
        &mut self,
        canvas: &mut WindowCanvas,
        target: &Rect,
        target_facing: Facing
    ) -> &mut Self{
        self.timer += self.elapsed();
        if self.timer >= self.interval {
            self.step().reset_zero();
        }
        let need_horizontal = target_facing != self.facing();
        self.play(
            canvas,
            None,
            target,
            std::f64::consts::PI / 2.0,
            None,
            need_horizontal,
            false
        ).new_clock()
    }

    fn play(
        &mut self,
        canvas:&mut WindowCanvas,
        src: Option<Rect>,
        target:&Rect,
        angle: f64,
        center: Option<FPoint>,
        flip_h: bool, //水平翻转
        flip_v: bool //竖直翻转
    ) -> &mut Self {
        canvas.copy_ex(
            self.picture(),
            None,
            *target,
            angle,
            None,
            flip_h,
            flip_v
        ).unwrap();
        self
    }
}

//获取数据的方法集合
impl<'a> AnimePlayer<'a> {
    pub fn picture(&self) -> &Texture<'a> {
        self.anime.picture(self.index)
    }

    pub fn elapsed(&self) -> Duration {
        self.clock.elapsed()
    }

    pub fn length(&self) -> usize {
        self.anime.length()
    }

    pub fn facing(&self) -> Facing{
        self.anime.facing()
    }
}

//更新数据的方法集合
impl<'a> AnimePlayer<'a> {
    pub fn new_clock(&mut self) -> &mut Self {
        self.clock = Instant::now();
        self
    }

    pub fn step(&mut self) -> &mut Self {
        self.index += 1;
        self.index %= self.length();
        self
    }

    pub fn reset_zero(&mut self) -> &mut Self {
        self.timer = Duration::ZERO;
        self
    }
}