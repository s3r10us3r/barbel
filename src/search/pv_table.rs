use crate::moving::mv::Move;

pub struct PvTable {
    n: usize,
    table: Box<[Move]>,
}

impl PvTable {
    pub fn new(n: usize) -> Self {
        let size = (n * n + n) / 2;
        PvTable {
            n,
            table: vec![Move::null(); size].into_boxed_slice(),
        }
    }

    pub fn update(&mut self, ply: usize, mv: Move) {
        self.table[self.index(ply, 0)] = mv;
        if ply + 1 < self.n {
            self.movcpy(self.index(ply, 1), self.index(ply + 1, 0), self.n - ply - 1);
        }
    }

    pub fn get_best(&self, ply: usize) -> Move {
        self.table[self.index(ply, 0)].clone()
    }

    pub fn get_pv_string(&self) -> String {
        let mut i = 0;
        let mut res = Vec::new();
        while i < self.n && !self.table[self.index(0, i)].is_null() {
            let mv = self.table[self.index(0, i)].clone();
            res.push(mv);
            i += 1;
        }
        let mut s = String::new();
        for mv in res {
            s.push_str(mv.to_str().as_str());
            s.push_str(" ");
        }
        s
    }

    fn movcpy(&mut self, mut target: usize, mut source: usize, mut n: usize) {
        while n != 0 && !self.table[source].is_null() {
            n -= 1;
            self.table[target] = self.table[source].clone();
            target += 1;
            source += 1;
        }
    }

    fn index(&self, ply: usize, col: usize) -> usize {
        ply * (2 * self.n - ply + 1) / 2 + col
    }
}
