//! 单个方块的状态信息

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
lazy_static! {
    static ref COLORTURN: Arc<Mutex<u8>> = Arc::new(Mutex::new(8));
}

// TODO START
// 让Tetris具备某一个trait，从而支持让tetris类型的对象可以直接通过等号进行赋值的同时，不会失去原先的所有权
// 即a和b都是tetris对象，令let a = b; b不会因为失去所有权而失效
// 提示：可以用derive快速实现trait
#[derive(Clone)]
pub struct Tetris {
    /// 0-7分别表示 TSZJLIO型方块
    pub kind: usize,
    /// 表示了在游戏中的位置，分别为x和y轴，认为position[0]是纵方向，position[1]是横方向
    pub position: [i32; 2],
    /// 表示了方块在游戏中的指向，0为初始方向，1-3依次顺时针旋转90°
    pub direc: usize,
    /// 表示了方块在游戏中的颜色
    pub color: u8,
}
// TODO END

impl Tetris {
    pub fn new() -> Self {
        Tetris {
            kind: (rand::random::<u8>() % 7) as usize,
            position: [-2, 5],
            direc: (rand::random::<u8>() % 4) as usize,
            color: {
                let mut color_turn_lock = COLORTURN.lock().unwrap();
                *color_turn_lock = (*color_turn_lock + 1) % 8;
                *color_turn_lock + 1
            },
        }
    }
    /// 方块右变形时的方向改变
    pub fn turn_right(&mut self) {
        self.direc += 1;
        self.direc %= 4;
    }
    /// 加上这个标注代表这个函数允许不被使用，否则当一个函数定义了但未被使用，则会报错
    #[allow(unused)]
    /// 方块左变形时的方向改变
    pub fn turn_left(&mut self) {
        // TODO START
        unimplemented!();
        // TODO FINISH
    }
}
