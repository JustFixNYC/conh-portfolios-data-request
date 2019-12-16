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

    fn populate_portfolio(&self, bbl: BBL, portfolio: &mut HashSet<BBL>) {
        if !portfolio.insert(bbl) {
            panic!("Assertion failure, did not expect BBL {} to be in portfolio!", bbl);
        }
        match self.bbls.get(&bbl) {
            Some(assoc_bbls) => {
                for assoc_bbl in assoc_bbls.iter() {
                    if !portfolio.contains(assoc_bbl) {
                        self.populate_portfolio(*assoc_bbl, portfolio);
                    }
                }
            },
            None => panic!("Assertion failure, no information about BBL {}!", bbl),
        }
    }

    pub fn get_portfolios(&self) -> Vec<HashSet<BBL>> {
        let mut results: Vec<HashSet<BBL>> = Vec::new();
        let mut unvisited_bbls: HashSet<BBL> = HashSet::new();
        unvisited_bbls.extend(self.bbls.keys());

        loop {
            match unvisited_bbls.iter().next() {
                Some(bbl) => {
                    let mut portfolio = HashSet::new();
                    self.populate_portfolio(*bbl, &mut portfolio);
                    for portfolio_bbl in portfolio.iter() {
                        unvisited_bbls.remove(portfolio_bbl);
                    }
                    results.insert(0, portfolio);
                },
                None => break,
            }
        }

        results
    }
}
