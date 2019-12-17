use super::bbl::BBL;
use std::collections::{HashSet, HashMap};

pub struct PortfolioMap {
    pub portfolios: Vec<Vec<BBL>>,
    pub bbl_mapping: HashMap<BBL, usize>,
}

pub struct PortfolioBuilder {
    bbls: HashMap<BBL, HashSet<BBL>>,
}

impl PortfolioBuilder {
    pub fn new() -> PortfolioBuilder {
        PortfolioBuilder { bbls: HashMap::new() }
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

    fn get_portfolio(&self, bbl: &BBL) -> Vec<BBL> {
        let mut portfolio_set = HashSet::new();
        self.populate_portfolio(*bbl, &mut portfolio_set);
        let mut portfolio = Vec::with_capacity(portfolio_set.len());
        portfolio.extend(portfolio_set.iter());
        portfolio.sort();
        portfolio
    }

    pub fn get_portfolios(&self) -> PortfolioMap {
        let mut portfolios: Vec<Vec<BBL>> = Vec::new();
        let mut bbl_mapping: HashMap<BBL, usize> = HashMap::with_capacity(self.bbls.len());
        let mut bbls_to_visit: Vec<BBL> = Vec::with_capacity(self.bbls.len());
        let mut i = 0;
        bbls_to_visit.extend(self.bbls.keys());
        bbls_to_visit.sort();

        for bbl in bbls_to_visit.iter() {
            if !bbl_mapping.contains_key(&bbl) {
                let portfolio = self.get_portfolio(bbl);
                for visited_bbl in portfolio.iter() {
                    bbl_mapping.insert(*visited_bbl, i);
                }
                portfolios.push(portfolio);
                i += 1;
            }
        }

        PortfolioMap { portfolios, bbl_mapping }
    }
}
