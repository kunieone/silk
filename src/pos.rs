#[derive(Clone, Copy, Debug)]
pub struct Pos {
    row: isize,
    col: isize,
}

impl Pos {
    pub fn new() -> Self {
        Self { row: 0, col: 0 }
    }
    pub fn row_move(&mut self, step: isize) {
        self.row += step;
    }
    pub fn col_move(&mut self, step: isize) {
        self.col += step;
    }
    pub fn update(&mut self, c: char) {
        if c == '\n' {
            self.row += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
    }
    pub fn tup(&self) -> (isize, isize) {
        (self.row, self.col)
    }
}
#[derive(Clone, Copy)]
pub struct Foots {
    pub last_pos: Pos,
    pub now_pos: Pos,
}

impl Foots {
    pub fn new() -> Self {
        Self {
            last_pos: Pos::new(),
            now_pos: Pos::new(),
        }
    }
    // 更新前脚。
    pub fn update(&mut self, c: char) {
        self.now_pos.update(c)
    }
    //记录，后脚跟上。返回后脚。
    pub fn sign_pos(&mut self) -> Pos {
        // println!("前脚:{:?},后脚{:?}", self.last_pos, self.now_pos);
        let sign = self.last_pos.clone();
        self.last_pos = self.now_pos;
        // self.last_pos = self.now_pos;
        sign
    }
}
