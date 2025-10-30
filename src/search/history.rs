use crate::moving::mv::Move;

pub struct HistoryTable {
    table: Box<[i32; 1 << 16]>
}

impl HistoryTable {
    pub fn new() -> Self {
        HistoryTable { table: Box::new([0; 1<< 16]) }
    }

    pub fn get_val(&self, mv: &Move) -> i32 {
        let index = mv.get_value() as usize;
        self.table[index]
    }

    pub fn add(&mut self, mv: &Move, depth_left: i32) {
        let index = mv.get_value() as usize;
        self.table[index] += (depth_left * depth_left) + 1;
    }
}
