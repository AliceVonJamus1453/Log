use sdl3::rect::{Point, Rect};
use sdl3::render::WindowCanvas;
use crate::base::anime_player::AnimePlayer;

pub struct Charactar<'a> {
    entity: Rect,
    anime_player: AnimePlayer<'a>
}

//构造函数的方法集合
impl<'a> Charactar<'a> {
    pub fn new(entity:Rect, anime_player: AnimePlayer<'a>) -> Self {
        Charactar {
            entity,
            anime_player
        }
    }
}

impl<'a> Charactar<'a> {
    pub fn normal(&mut self, canvas:&mut WindowCanvas) -> &mut Self {
        self.anime_player.normal(canvas,&self.entity);
        self
    }
}

//获取数据的方法集合
impl<'a> Charactar<'a> {
    pub fn x(&self) -> i32 {
        self.entity.x()
    }

    pub fn y(&self) -> i32 {
        self.entity.y()
    }

    pub fn center(&self) -> Point {
        self.entity.center()
    }
}

//更新数据的方法集合
impl<'a> Charactar<'a> {
    pub fn set_x(&mut self, target:i32) -> &mut Self {
        self.entity.set_x(target);
        self
    }

    pub fn set_y(&mut self, target:i32) -> &mut Self {
        self.entity.set_y(target);
        self
    }

    pub fn center_on(&mut self, target:Point) -> &mut Self {
        self.entity.center_on(target);
        self
    }
}