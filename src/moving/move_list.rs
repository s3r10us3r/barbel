use std::ops::Index;

use crate::moving::mv::Move;

pub struct MoveList {
    moves: [Move; 218],
    count: usize,
}

impl Default for MoveList {
    fn default() -> Self {
        MoveList {
            moves: std::array::from_fn(|_| Move::new_null_mv()),
            count: 0,
        }
    }
}

impl Index<usize> for MoveList {
    type Output = Move;

    fn index(&self, i: usize) -> &Self::Output {
        &self.moves[i]
    }
}

impl MoveList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn moves(&mut self) -> &mut [Move] {
        &mut self.moves
    }

    pub fn push_move(&mut self, mv: Move) {
        self.moves[self.count] = mv;
        self.count += 1;
    }

    pub fn reset(&mut self) {
        self.count = 0;
    }

    pub fn remove(&mut self, i: usize) {
        self.count -= 1;
        self.moves.swap(self.count, i);
    }

    pub fn get_count(&self) -> usize {
        self.count
    }

    pub fn get_move(&self, i: usize) -> &Move {
        &self.moves[i]
    }
}

pub struct Iter<'a> {
    moves: &'a [Move],
    index: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Move;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.moves.len() {
            let result = &self.moves[self.index];
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl <'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index > 0 {
            let res = &self.moves[self.index];
            self.index -= 1;
            Some(res)
        } else {
            None
        }
    }
}

impl <'a> ExactSizeIterator for Iter<'a> {
    fn len(&self) -> usize {
        self.moves.len() - self.index
    }
}

impl MoveList {
    pub fn iter(&self) -> Iter {
        Iter {
            moves: &self.moves[..self.count],
            index: 0,
        }
    }
}
