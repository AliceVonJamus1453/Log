use crate::base::facing::Facing;
pub struct MoveController {
    move_state: MoveState,
    speed: Option<f32>
}

//构造函数的集合
impl MoveController {
    pub fn new(speed:Option<f32>) -> Self {
        MoveController {
            move_state: MoveState::Stop,
            speed
        }
    }
}

//获取数据的方法集合
impl MoveController {
    pub fn state(&self) -> MoveState {
        self.move_state.clone()
    }
}


#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MoveState {
    Moving,
    Stop,
}