
use std::time::Instant;

use crate::{search::alpha_beta::Searcher, tests::epd::Epd};

pub struct NpsResult {
    pub time: u128,
    pub nodes: u64
}

pub fn make_nps(positions: &str, search_depth: i32) -> NpsResult {
    let mut nodes = 0u64;
    let mut time = 0u128; //in ms
    for position in positions.lines() {
        let position = position.trim();
        let mut epd = Epd::new(position);
        let mut searcher = Searcher::new();

        let start = Instant::now();
        //searcher.search_to_depth(&mut epd.position, search_depth);
        searcher.search_to_time(&mut epd.position, 2000, false);
        let duration = start.elapsed();
        time += duration.as_millis();
        nodes += searcher.get_nodes_searched() as u64;
    }
    NpsResult { time, nodes }
}
