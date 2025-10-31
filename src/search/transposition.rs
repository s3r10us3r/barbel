use crate::moving::mv::Move;

const K: usize = 23;
const GEN_DIFF: i32 = 5;

#[derive(Clone, Copy, Default)]
pub enum TTEntryType {
    #[default]
    Exact,
    Lower,
    Upper
}

// this has space left, so option does not increase its size
// if it were to change check for option size
#[derive(Clone, Copy, Default)]
pub struct Entry {
    pub key: u64,
    pub depth_left: i32,
    pub score: i32,
    pub generation: i32,
    pub entry_type: TTEntryType,
    pub best_move: Move
}


pub struct TTable {
    mask: usize,
    table: Box<[Option<Entry>]>,
}

//depth preferred for now
impl TTable {
    pub fn new() -> Self {
        let entries = 1 << K;
        TTable {
            mask: entries - 1,
            table: vec![None; entries].into_boxed_slice(),
        }
    }

    pub fn probe(&self, key: u64) -> Option<Entry> {
        let index = (key as usize) & self.mask;
        match self.table[index] {
            Some(e) if e.key == key => Some(e),
            _ => None
        }
    }

    pub fn store(&mut self, new: Entry) {
        let index = (new.key as usize) & self.mask;
        let existing = &self.table[index];
        if existing.is_none() || existing.is_some_and(|e| e.depth_left <= new.depth_left || new.generation - e.generation >= GEN_DIFF) {
            self.table[index] = Some(new);
        }
    }
}
