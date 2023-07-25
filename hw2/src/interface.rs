//! 负责根据给定状态来绘制游戏内部的文字界面
use std::io::stdout;

use crate::{frame::FrameWork, tetris::Tetris};
use crossterm::{
    cursor::{MoveTo, MoveToNextLine},
    execute,
    style::*,
};

fn clear() {
    let _t = execute!(stdout(), MoveTo(0, 0));
}

pub const TETRIS: [[[i32; 2]; 6]; 7] = [
    // ? 数据格式：前两个为[x上限下限]，[y上限下限]，后面余下的为相对于旋转中心的坐标
    [[1, 0], [1, -1], [0, 0], [0, -1], [0, 1], [1, 0]], // T
    [[1, 0], [1, -1], [0, 0], [0, 1], [1, 0], [1, -1]], // S
    [[1, 0], [1, -1], [0, 0], [0, -1], [1, 0], [1, 1]], // Z
    [[1, -1], [0, -1], [0, 0], [-1, 0], [1, 0], [1, -1]], // J
    [[1, -1], [0, 1], [0, 0], [-1, 0], [1, 0], [1, 1]], // L
    [[1, -2], [0, 0], [0, 0], [-1, 0], [1, 0], [-2, 0]], // I
    [[1, 0], [1, 0], [0, 0], [1, 0], [0, 1], [1, 1]],   // O
];

/// 根据方块的类型和方向来获取方块每一个像素的实际变化后坐标
///
/// 根据中心的移动加上方块的方向确定实际坐标
///
/// 当前的实现是比较简陋的，可能存在一定的bug
pub fn xt_yt(points: &[i32; 2], t: &Tetris) -> (i32, i32) {
    (
        t.position[0] as i32
            + match t.direc {
                0 => points[0],
                1 => -points[1],
                2 => -points[0],
                _ => points[1],
            } as i32,
        t.position[1] as i32
            + match t.direc {
                0 => points[1],
                1 => points[0],
                2 => -points[1],
                _ => -points[0],
            } as i32,
    )
}

/// 游戏状态枚举
#[derive(PartialEq, Clone)]
pub enum GameState {
    /// 游戏已经停止
    Stopped,
    /// 游戏正在进行
    Playing,
    /// 游戏暂停
    Pausing,
}
/// 绘制文字
fn write(interface: &mut Vec<Vec<char>>, left: usize, top: usize, words: String) {
    assert!(left + words.len() < interface[0].len());
    for (i, ch) in words.chars().enumerate() {
        interface[top][i + left] = ch;
    }
}

/// 绘制带风格的文字，包括颜色、大小、粗细等等
fn write_styled(
    interface: &mut Vec<Vec<StyledContent<char>>>,
    left: usize,
    top: usize,
    words: String,
) {
    assert!(left + words.len() < interface[0].len());
    for (i, ch) in words.chars().enumerate() {
        interface[top][i + left] = style(ch);
    }
}

/// 游戏界面结构体
pub struct InterFace {
    /// 页面的文字集合，其中Vec<char>代表一句完整的语句
    pub interface: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl InterFace {
    /// 创建新的文字界面
    pub fn new() -> Self {
        let mut temp = InterFace {
            width: 70,
            height: 30,
            interface: vec![],
        };
        let mut frame = FrameWork::new(temp.height, temp.width);
        frame.rdraw(0, 29, 0, 39);
        frame.rdraw(6, 27, 9, 30);
        frame.rdraw(4, 12, 48, 61);
        temp.interface = frame.get_vec();
        write(&mut temp.interface, 49, 2, "Next Tetris".to_string());
        write(&mut temp.interface, 43, 14, "Operations:".to_string());
        write(&mut temp.interface, 43, 16, "space:".to_string());
        write(&mut temp.interface, 47, 17, "pause the game".to_string());
        write(&mut temp.interface, 43, 18, "q:".to_string());
        write(&mut temp.interface, 47, 19, "quit the game".to_string());
        write(&mut temp.interface, 43, 20, "↑↓←→:".to_string());
        write(
            &mut temp.interface,
            47,
            21,
            "control the tertris".to_string(),
        );
        temp
    }
    /// 通过调用相关函数，直接绘制出游戏边界与文字
    pub fn show_frame(
        &self,
        t: &Tetris,
        next: &Tetris,
        blockes: &[[u8; 10]; 20],
        state: &GameState,
        scores: u128,
    ) {
        clear();
        let mut interface = vec![];
        for lines in &self.interface {
            let mut line = vec![];
            for &ch in lines {
                line.push(style(ch))
            }
            interface.push(line);
        }

        // 绘制边框
        for points in &TETRIS[t.kind][2..] {
            let (xt, yt) = xt_yt(points, t);
            if xt >= 0 && xt < 20 && yt >= 0 && yt < 10 {
                interface[xt as usize + 7][(yt as usize) * 2 + 10] =
                    style('█').with(Color::AnsiValue(t.color));
                interface[xt as usize + 7][(yt as usize) * 2 + 11] =
                    style('█').with(Color::AnsiValue(t.color));
            }
        }
        for points in &TETRIS[next.kind][2..] {
            interface[(points[0] + 8) as usize][(points[1] * 2 + 54) as usize] =
                style('█').with(Color::AnsiValue(next.color));
            interface[(points[0] + 8) as usize][(points[1] * 2 + 55) as usize] =
                style('█').with(Color::AnsiValue(next.color));
        }
        for x in 0..20 {
            for y in 0..10 {
                if blockes[x][y] >= 1 {
                    interface[x as usize + 7][(y as usize) * 2 + 10] =
                        style('█').with(Color::AnsiValue(blockes[x][y]));
                    interface[x as usize + 7][(y as usize) * 2 + 11] =
                        style('█').with(Color::AnsiValue(blockes[x][y]));
                }
            }
        }

        // TODO START
        // 修改下列代码，使得其可以正常通过编译

        // words存储当前状态
        let words = match state {
            GameState::Playing => "Playing",
            GameState::Stopped => "You loose!",
            GameState::Pausing => "Pausing",
        };
        // 绘制当前状态
        write_styled(interface, 17, 5, words);

        // 定义一个字符串，格式为"Scores: xx"，其中xx为当前总分，即传入的参数scores
        let scores_str = "";
        // 绘制当前总分
        write_styled(interface, 16, 3, scores_str);

        // TODO END
        for line in &interface {
            for &ch in line {
                print!("{}", ch);
            }
            let _ = execute!(stdout(), MoveToNextLine(1));
        }
    }
}
