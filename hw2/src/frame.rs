//! 负责绘制游戏外围框架

/// 用于绘制游戏外围框架的结构体
///
/// 与内部的游戏逻辑无关
pub struct FrameWork {
    /// 页面高度
    height: usize,
    /// 页面宽度
    width: usize,
    stringify: Vec<Vec<usize>>,
}

impl FrameWork {
    /// 新建一个游戏框架，指定对应的宽度和高度
    pub fn new(horizontal: usize, vertical: usize) -> Self {
        let mut temp = FrameWork {
            height: horizontal,
            width: vertical,
            stringify: vec![vec![0; vertical]; horizontal],
        };
        temp.draw(0, 0, vertical - 1, horizontal - 1);
        temp
    }
    /// 用于绘制游戏外围框架的函数
    pub fn rdraw(&mut self, up: usize, down: usize, left: usize, right: usize) {
        // Real Draw with real x, y crossover
        if up >= self.height
            || down >= self.height
            || left >= self.width
            || right >= self.width
            || left == right
            || up == down
        {
            eprintln!("\nThe Parameters You Had Given Is Incorrent!Function Draw will not work!\nTraceBack:\n\tleft:{} right:{} width_limit:{}\n\tup:{} down:{} height_limit:{}\n", left, right, self.width, up, down, self.height);
            return;
        }
        for i in (up + 1)..down {
            self.stringify[i][left] |= 3;
            self.stringify[i][right] |= 3;
        }
        for i in (left + 1)..right {
            self.stringify[up][i] |= 12;
            self.stringify[down][i] |= 12;
        }
        self.stringify[up][left] |= 6;
        self.stringify[up][right] |= 10;
        self.stringify[down][left] |= 5;
        self.stringify[down][right] |= 9;
    }
    fn draw(&mut self, x: usize, y: usize, width: usize, height: usize) {
        self.rdraw(
            self.height - 1 - y - height,
            self.height - 1 - y,
            x,
            x + width,
        );
    }
    pub fn get_vec(&self) -> Vec<Vec<char>> {
        let vs = [
            ' ', '上', '下', '║', '左', '╚', '╔', '╠', '右', '╝', '╗', '╣', '═', '╩', '╦', '╬',
        ];
        let mut ret = vec![];
        for s in self.stringify.iter() {
            ret.push(s.iter().map(|&x| vs[x]).collect::<Vec<char>>());
        }
        ret
    }
}
