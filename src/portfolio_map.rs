use super::bbl::BBL;
use std::collections::{HashSet, HashMap};

pub struct PortfolioMap {
    bbls: HashMap<BBL, HashSet<BBL>>,
}

impl PortfolioMap {
    pub fn new() -> PortfolioMap {
        PortfolioMap { bbls: HashMap::new() }
    }

    fn associate_one_way(&mut self, a: &BBL, b: &BBL) {
        let assoc = self.bbls.entry(*a).or_insert_with(|| HashSet::new());
        assoc.insert(*b);
    }

    pub fn associate(&mut self, a: &BBL, b: &BBL) {
        if a != b {
            self.associate_one_way(a, b);
            self.associate_one_way(b, a);
        }
    }

    pub fn num_bbls(&self) -> usize {
        self.bbls.len()
    }
}
