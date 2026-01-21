use crate::moving::mv::Move;

const KILLER_COUNT: usize = 2;
const MAX_PLY: usize = 100;

pub struct KillerTable {
    table: [[Move; KILLER_COUNT]; MAX_PLY]
}

impl KillerTable {
    pub fn new() -> Self {
        let table = [[Move::null(); KILLER_COUNT]; MAX_PLY];
        KillerTable { table }
    }

    pub fn update(&mut self, ply: i32, mv: Move) {
        let ply = std::cmp::min(ply as usize, MAX_PLY - 1);

        if self.table[ply][0].is_null() {
            self.table[ply][1] = self.table[ply][0];
        }
        self.table[ply][0] = mv;
    }

    pub fn is_killer(&self, ply: i32, mv: &Move) -> bool {
        let ply = ply as usize;
        ply < MAX_PLY && (self.table[ply][0] == *mv || self.table[ply][1] == *mv)
    }
}
