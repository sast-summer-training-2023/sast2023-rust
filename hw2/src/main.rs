//! 游戏主逻辑
use std::{
    io::stdout,
    process::exit,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crossterm::{
    cursor::Hide,
    event::{read, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    Result,
};

pub mod frame;
mod game;
mod interface;
use interface::{GameState, InterFace};
mod tetris;
use game::Game;
use lazy_static::lazy_static;
use tetris::Tetris;

// 使用lazy_static包来实例化一个全局变量
lazy_static! {
    // 用于记录当前线程数，通过互斥锁的形式防止线程同时执行，保证0.5s内只会移动一次
    static ref THREAD_COUNT: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    // 初始化一个游戏实例
    static ref GAME: Arc<Mutex<Game>> = Arc::new(Mutex::new(Game {
        state: GameState::Stopped,
        interface: InterFace {
            width: 70,
            height: 30,
            interface: vec![],
        },
        blockes: [[0; 10]; 20],
        curter: Tetris {
            kind: 0,
            position: [-2, 5],
            direc: 0,
            color: 0,
        },
        nxtter: Tetris {
            kind: 0 as usize,
            position: [-2, 5],
            direc: 0 as usize,
            color: 0,
        },
        scores: 0,
    }));
}

/// 改变游戏状态的函数
///
/// 若处于运行状态，则随时间推移，方块向下移动，固定每0.5s向下移动一次
///
/// 若处于暂停状态，则停止运动
///
/// 原理较为复杂，大家可以不用去细究
fn trans() {
    // 这里用到了互斥锁以及线程的概念，保证向下移动的时间固定为0.5s

    // 如果trans在0.5s内被调用了100次，那么不会是0.5s内移动了100次，而是每隔0.5s移动一次，一共移动100次
    let mut begin_new_thread = false;
    {
        let mut game_lock = GAME.lock().unwrap();
        if game_lock.state == GameState::Playing {
            // 若当前处于运行状态，则暂停游戏
            game_lock.state = GameState::Pausing;
            game_lock.show_all();
        } else {
            // 否则若是未开始游戏，则开始一盘新的游戏
            if game_lock.state == GameState::Stopped {
                *game_lock = Game::new();
            }

            // TODO START
            // 不管是暂停状态还是停止状态，都直接重新进入运行状态
            // 修改game_lock.state使其成为运行状态
            unimplemented!();
            // TODO END
            game_lock.show_all();
            begin_new_thread = true
        }
    }
    if begin_new_thread {
        thread::spawn(move || {
            let _thread_holder = THREAD_COUNT.lock().unwrap();
            loop {
                std::thread::sleep(Duration::from_millis(500));
                let mut game_lock = GAME.lock().unwrap();
                if game_lock.state != GameState::Playing {
                    return;
                }
                game_lock.down();
                game_lock.show_all();
            }
        });
    }
}
fn main() -> Result<()> {
    // 启用可操作终端，用到了外部的包
    if let Err(_) = enable_raw_mode() {
        println!("Your terminal does not support raw mode, please try another terminal or visit https://docs.rs/crossterm/0.23.0/crossterm/terminal/#raw-mode for more help.\
        Please Not Run This Program Directly in CLion/Intellij IDEA 's Run Tag!");
        exit(0);
    }
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    // 清空cmd界面
    let _ = execute!(stdout, Clear(ClearType::All));
    trans();
    loop {
        // 接受键盘输入，并且进行游戏演化
        let event: crossterm::event::Event = read()?;
        if let Event::Key(_keyevent) = event {
            match _keyevent.code {
                // 如果按到的是字符键
                KeyCode::Char(ch) => match ch {
                    // 按到了q会退出游戏
                    'q' => break,
                    // 按到空格，则调用trans函数，改变游戏状态
                    ' ' => {
                        // TODO START
                        unimplemented!();
                        // TODO FINISH
                    }
                    _ => (),
                },
                // 按↑（👆）键
                // 只有当前处于游戏状态时，按下按键才能有对应的反应
                KeyCode::Up => {
                    let mut game_lock = GAME.lock().unwrap();
                    if game_lock.state == GameState::Playing {
                        // 进行变形
                        game_lock.turn();
                        game_lock.show_all();
                    }
                }
                // 按↓（👇）键
                KeyCode::Down => {
                    let mut game_lock = GAME.lock().unwrap();
                    if game_lock.state == GameState::Playing {
                        // 往下走一格
                        game_lock.down();
                        game_lock.show_all();
                    }
                }
                // 按←（👈）键
                KeyCode::Left => {
                    // TODO START
                    let game_lock = GAME.lock().unwrap();
                    if game_lock.state == GameState::Playing {
                        unimplemented!();
                    }
                    // TODO FINISH
                }
                // 按→（👉）键
                KeyCode::Right => {
                    let mut game_lock = GAME.lock().unwrap();
                    if game_lock.state == GameState::Playing {
                        game_lock.shift(1);
                        game_lock.show_all();
                    }
                }
                _ => {}
            }
        }
    }
    execute!(stdout, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
