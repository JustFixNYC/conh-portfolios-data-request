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

    fn get_portfolio(&self, root_bbl: BBL) -> HashSet<BBL> {
        let mut portfolio: HashSet<BBL> = HashSet::new();
        let mut bbls_to_visit = Vec::new();
        bbls_to_visit.insert(0, root_bbl);
        loop {
            match bbls_to_visit.pop() {
                Some(bbl) => {
                    if !portfolio.insert(bbl) {
                        continue;
                    }
                    match self.bbls.get(&bbl) {
                        Some(assoc_bbls) => {
                            for assoc_bbl in assoc_bbls.iter() {
                                if !portfolio.contains(assoc_bbl) {
                                    bbls_to_visit.insert(0, *assoc_bbl);
                                }
                            }
                        },
                        None => panic!("Assertion failure, no information about BBL {}!", bbl),
                    }
                },
                None => break,
            }
        }
        portfolio
    }

    pub fn get_portfolios(&self) -> Vec<HashSet<BBL>> {
        let mut results: Vec<HashSet<BBL>> = Vec::new();
        let mut unvisited_bbls: HashSet<BBL> = HashSet::new();
        unvisited_bbls.extend(self.bbls.keys());

        loop {
            match unvisited_bbls.iter().next() {
                Some(bbl) => {
                    let portfolio = self.get_portfolio(*bbl);
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
