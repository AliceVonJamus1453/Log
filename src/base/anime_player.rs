use std::time::Duration;
use crate::base::anime::Anime;

pub struct AnimePlayer<'a> {
    anime: &'a Anime<'a>,
    timer: Duration, //计时参数
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
            timer: Duration::default()
        }
    }
}