//! 游戏状态演化控制
//!
//! 实现方块的移动与变形
//!
//! 存储相关的状态
use crate::{
    interface::{xt_yt, GameState, InterFace, TETRIS},
    tetris::Tetris,
};

pub struct Game {
    /// 游戏当前状态
    pub state: GameState,
    /// 内部文字
    pub interface: InterFace,
    /// 主界面的每一个像素点的状态
    pub blockes: [[u8; 10]; 20],
    /// 当前方块
    pub curter: Tetris,
    /// 下一个方块
    pub nxtter: Tetris,
    /// 当前分数
    pub scores: u128,
}

impl Game {
    /// 新建一个新的游戏
    pub fn new() -> Self {
        Game {
            state: GameState::Stopped,
            interface: InterFace::new(),
            blockes: [[0; 10]; 20],
            curter: Tetris::new(),
            nxtter: Tetris::new(),
            scores: 0,
        }
    }
    /// 当前方块向下移动
    pub fn down(&mut self) {
        let mut bottom = false;
        // 先判断是否到达了底部，即该方块不能再移动了
        for points in &TETRIS[self.curter.kind][2..] {
            let (xt, yt) = xt_yt(points, &self.curter);
            let xt = xt + 1;
            if !(xt < 0 || xt < 20 && self.blockes[xt as usize][yt as usize] == 0) {
                bottom = true;
                break;
            }
        }
        if bottom {
            // 若到达了底部则进行判定
            for points in &TETRIS[self.curter.kind][2..] {
                let (xt, yt) = xt_yt(points, &self.curter);
                if xt < 0 || yt < 0 {
                    // 若溢出了，就是直接顶到了天花板，则游戏终止
                    self.state = GameState::Stopped;
                    self.show_all();
                    return;
                }
                self.blockes[xt as usize][yt as usize] = self.curter.color;
            }
            // 判定是否有可以消除的行
            let mut cleanpath = 0;
            for x in 0..20 {
                let mut filled = true;
                for y in 0..10 {
                    filled = filled & (self.blockes[x][y] != 0);
                }
                if filled {
                    // 若有可以消除的行，则进行消除
                    cleanpath += 1;
                    for i in (1..=x).rev() {
                        self.blockes[i] = self.blockes[i - 1];
                    }
                    self.blockes[0] = [0; 10];
                }
            }
            if cleanpath > 0 {
                self.scores += 20u128.pow(cleanpath);
            }

            // TODO START

            // 更新当前的方块为下一个方块
            // 注意直接改变所有权会进行报错，因为此时会让self.nxtter失效，但是我们不允许一个有效对象的某一个成员直接失效
            // 所以我们需要为Tetris实现某一个trait
            self.curter = self.nxtter;

            // 使用Tetris的生成函数生成下一个方块，并绑定到self.nxtter上
            unimplemented!();

            // TODO END
        } else {
            // 若没有到达底部，则向下移动
            self.curter.position[0] += 1;
        }
    }
    /// 当前方块进行移动，包括向左、向右
    ///
    /// moved为-1代表向左移动
    ///
    /// moved为1代表向右移动
    pub fn shift(&mut self, moved: i32) {
        for points in &TETRIS[self.curter.kind][2..] {
            let (xt, yt) = xt_yt(points, &self.curter);
            let yt = yt + moved;
            if (yt < 0 || yt >= 10)
                || !(xt < 0 || xt < 20 && self.blockes[xt as usize][yt as usize] == 0)
            {
                // 如果移动的时候撞墙了，则不移动
                // 撞墙包括左右的实体墙以及其他存在的方块
                return;
            }
        }
        self.curter.position[1] += moved;
    }
    /// 当前方块进行变形旋转
    pub fn turn(&mut self) {
        // 先进行向右旋转，再判断旋转后是否合法
        let _ = &self.curter.turn_right();
        for points in &TETRIS[self.curter.kind][2..] {
            let (xt, yt) = xt_yt(points, &self.curter);
            if (yt < 0 || yt >= 10)
                || !(xt < 0 || xt < 20 && self.blockes[xt as usize][yt as usize] == 0)
            {
                // TODO START
                // 若不合法，则旋转回去，即复原
                // 提示：向右旋转的反面是什么呢
                // TODO FINISH
                return;
            }
        }
    }
    /// 显示当前的游戏状态
    pub fn show_all(&self) {
        self.interface.show_frame(
            &self.curter,
            &self.nxtter,
            &self.blockes,
            &self.state,
            self.scores,
        );
    }
}
