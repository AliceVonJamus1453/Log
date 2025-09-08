use sdl3::rect::{Point, Rect};
use sdl3::render::WindowCanvas;
use crate::base::anime_player::AnimePlayer;
use crate::base::move_controller::{MoveController, MoveState};
use crate::base::facing::Facing;

pub struct Character<'a> {
    entity: Rect,
    anime_player: AnimePlayer<'a>,
    move_controller: MoveController,
    facing: Facing
}

//构造函数的方法集合
impl<'a> Character<'a> {
    pub fn new(entity:Rect, anime_player: AnimePlayer<'a>, speed:Option<i32>) -> Self {
        Character {
            entity,
            anime_player,
            move_controller: MoveController::new(speed),
            facing: Facing::Right
        }
    }
}

//核心方法的集合
impl<'a> Character<'a> {
    pub fn run(&mut self, canvas:&mut WindowCanvas) -> &mut Self {
        match self.state() {
            MoveState::Stop => {
                self.normal_stop(canvas);
            }
            MoveState::Moving => {}
        }
        self
    }

    fn normal_stop(&mut self, canvas:&mut WindowCanvas) -> &mut Self {
        self.anime_player.normal_stop(canvas, &self.entity, self.facing());
        self
    }
}

//获取数据的方法集合
impl<'a> Character<'a> {
    pub fn x(&self) -> i32 {
        self.entity.x()
    }

    pub fn y(&self) -> i32 {
        self.entity.y()
    }

    pub fn center(&self) -> Point {
        self.entity.center()
    }

    pub fn state(&self) -> MoveState{
        self.move_controller.state()
    }

    pub fn facing(&self) -> Facing {
        self.facing.facing()
    }
}

//更新数据的方法集合
impl<'a> Character<'a> {
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
    
    pub fn set_facing(&mut self, target: Facing) -> &mut Self {
        self.facing = target;
        self
    }
}