const K: usize = 23;

#[derive(Clone, Copy, Default)]
pub enum TTEntryType {
    #[default]
    Exact,
    Lower,
    Upper
}

#[derive(Clone, Copy, Default)]
pub struct Entry {
    pub key: u64,
    pub depth_left: i32,
    pub score: i32,
    pub generation: i32,
    pub entry_type: TTEntryType,
}

pub struct TTable {
    mask: usize,
    table: Box<[Entry]>,
}

//depth preferred for now
impl TTable {
    pub fn new() -> Self {
        let entries = 1 << K;
        TTable {
            mask: entries - 1,
            table: vec![Entry::default(); entries].into_boxed_slice(),
        }
    }

    pub fn probe(&self, key: u64) -> Option<&Entry> {
        let index = (key as usize) & self.mask;
        let entry = &self.table[index];
        if entry.key == key {
            Some(entry)
        } else {
            None
        }
    }

    pub fn store(&mut self, new: Entry) {
        let index = (new.key as usize) & self.mask;
        let existing = &self.table[index];
        if existing.key == 0 || existing.depth_left < new.depth_left || new.generation - 5 >= existing.generation {
            self.table[index] = new;
        }
    }
}
