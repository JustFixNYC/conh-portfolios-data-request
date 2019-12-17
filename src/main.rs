use std::fs::File;
use std::io::BufReader;
use std::collections::HashSet;
use serde::{Deserialize};

mod bbl;
mod portfolio_map;

use bbl::BBL;
use portfolio_map::PortfolioBuilder;

const WOW_API_ROOT: &'static str = "https://whoownswhat.justfix.nyc/api";

#[derive(Debug, Deserialize)]
struct CONHRecord {
    #[serde(rename = "Borocode")]
    boro: u8,

    #[serde(rename = "Block")]
    block: u32,

    #[serde(rename = "Lot")]
    lot: u16,
}

impl CONHRecord {
    fn as_bbl(&self) -> BBL {
        BBL::new(self.boro, self.block, self.lot)
    }
}

#[derive(Debug, Deserialize)]
struct WOWAddrResult {
    bbl: String,
}

impl WOWAddrResult {
    fn as_bbl(&self) -> BBL {
        self.bbl.parse().unwrap()
    }
}

#[derive(Debug, Deserialize)]
struct WOWAddrResults {
    addrs: Vec<WOWAddrResult>,
}

#[derive(Debug, Deserialize)]
struct WOWAggResult {
    bldgs: String,
    units: Option<String>,
    topowners: Option<Vec<String>>,
    topcorp: Option<String>,
    topbusinessaddr: Option<String>,
    totalopenviolations: Option<String>,
    totalviolations: Option<String>,
}

#[derive(Debug, Deserialize)]
struct WOWAggResults {
    result: Vec<WOWAggResult>,
}

fn wow_address_api_url(bbl: &BBL) -> String {
    format!("{}/address?block={:05}&lot={:04}&borough={}", WOW_API_ROOT, bbl.block, bbl.lot, bbl.boro)
}

fn wow_aggregate_api_url(bbl: &BBL) -> String {
    format!("{}/address/aggregate?bbl={}", WOW_API_ROOT, bbl)
}

fn get_from_cache_or_download(filename: String, url: String) -> String {
    let path_str = format!("./data/wow/{}", filename);
    let path = std::path::Path::new(&path_str);
    if !path.exists() {
        let cache_dir = path.parent().unwrap();
        if !cache_dir.exists() {
            std::fs::create_dir_all(cache_dir).unwrap();
        }
        println!("Downloading {}.", url);
        let result = reqwest::get(&url).unwrap().text().unwrap();
        std::fs::write(path, result).unwrap();
    }
    std::fs::read_to_string(path).unwrap()
}

fn iter_conh_records() -> impl Iterator<Item = CONHRecord> {
    let file = File::open("./data/Certification_of_No_Harassment__CONH__Pilot_Building_List.csv").unwrap();
    let buf_reader = BufReader::new(file);
    let rdr = csv::Reader::from_reader(buf_reader);
    let records = rdr.into_deserialize::<CONHRecord>();
    records.map(|rec| rec.unwrap())
}

fn main() {
    let mut conh_bbls = HashSet::new();
    let mut portfolios = PortfolioBuilder::new();
    let mut row_count = 0;
    for (i, rec) in iter_conh_records().enumerate() {
        let bbl = rec.as_bbl();
        let row_num = i + 1;
        if row_num % 50 == 0 || row_num == 1 {
            println!("Processing row #{}.", row_num);
        }
        if !conh_bbls.insert(bbl) {
            println!("Warning: BBL {} exists at least twice in CONH candidates!", bbl);
        }
        let addr_info = get_from_cache_or_download(format!("{}-addr.json", bbl), wow_address_api_url(&bbl));
        let agg_info = get_from_cache_or_download(format!("{}-agg.json", bbl), wow_aggregate_api_url(&bbl));
        let addr_results: WOWAddrResults = serde_json::from_str(&addr_info).unwrap();
        for addr in addr_results.addrs.iter() {
            portfolios.associate(&bbl, &addr.as_bbl());
        }
        let agg_results: WOWAggResults = serde_json::from_str(&agg_info).unwrap();
        for _result in agg_results.result.iter() {
            // println!("  Units in portfolio: {:?}", &result.units);
        }
        row_count += 1;
    }
    println!("Found {} unique CONH BBLs over {} rows.", conh_bbls.len(), row_count);
    println!("Portfolios span a total of {} unique BBLs.", portfolios.num_bbls());
    let pmap = portfolios.get_portfolios();
    println!("Found {} disjoint portfolios.", pmap.portfolios.len());
}

#[test]
fn test_csv_is_parseable() {
    let mut found = false;
    for rec in iter_conh_records() {
        if rec.boro == 1 { found = true; }
    }
    assert_eq!(found, true);
}

#[test]
fn test_bbl_works() {
    let rec = CONHRecord { boro: 1, block: 5099, lot: 39 };
    assert_eq!(rec.as_bbl().to_string(), "1050990039");
}

#[test]
fn test_wow_address_api_url_works() {
    assert_eq!(wow_address_api_url(&BBL::new(1, 5099, 39)), "https://whoownswhat.justfix.nyc/api/address?block=05099&lot=0039&borough=1");
}

#[test]
fn test_wow_aggregate_api_url_works() {
    assert_eq!(wow_aggregate_api_url(&BBL::new(1, 5099, 39)), "https://whoownswhat.justfix.nyc/api/address/aggregate?bbl=1050990039");
}
