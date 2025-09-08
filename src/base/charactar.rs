use sdl3::rect::{Point, Rect};
use crate::base::anime::Anime;

pub struct Charactar<'a> {
    entity: Rect,
    anime: &'a Anime<'a>,
}

//构造函数的方法集合
impl<'a> Charactar<'a> {
    pub fn new(entity:Rect, anime: &'a Anime<'a>) -> Self {
        Charactar {
            entity,
            anime
        }
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