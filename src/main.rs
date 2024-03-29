use std::fs::File;
use std::io::BufReader;
use std::iter::FromIterator;
use std::collections::{HashSet, HashMap};
use serde::{Deserialize, Serialize};

mod bbl;
mod portfolio_map;

use bbl::BBL;
use portfolio_map::PortfolioBuilder;

const WOW_API_ROOT: &'static str = "https://whoownswhat.justfix.nyc/api";
const OUTPUT_FILENAME: &'static str = "./data/output.csv";

#[derive(Debug, Serialize)]
struct OutputRecord {
    #[serde(rename = "Building ID")]
    building_id: u32,

    #[serde(rename = "BIN")]
    bin: u32,

    #[serde(rename = "Street Address")]
    street_address: String,

    #[serde(rename = "Borocode")]
    boro: u8,

    #[serde(rename = "Block")]
    block: u32,

    #[serde(rename = "Lot")]
    lot: u16,

    #[serde(rename = "BBL")]
    bbl: String,

    #[serde(rename = "Number of buildings in portfolio (from WoW)")]
    bldgs: String,

    #[serde(rename = "Number of units in portfolio (from WoW)")]
    units: Option<String>,

    #[serde(rename = "Top owners (from WoW)")]
    topowners: String,

    #[serde(rename = "Top corporation name (from WoW)")]
    topcorp: Option<String>,

    #[serde(rename = "Top business address (from WoW)")]
    topbusinessaddr: Option<String>,

    #[serde(rename = "Virtual portfolio ID")]
    virtual_portfolio_id: usize,

    #[serde(rename = "Virtual portfolio BBL count")]
    virtual_portfolio_bbl_count: usize,
}

#[derive(Debug, Deserialize)]
struct CONHRecord {
    #[serde(rename = "Building ID")]
    building_id: u32,

    #[serde(rename = "BIN")]
    bin: u32,

    #[serde(rename = "Street Address")]
    street_address: String,

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

fn get_addr_results(bbl: BBL) -> WOWAddrResults {
    let addr_info = get_from_cache_or_download(format!("{}-addr.json", bbl), wow_address_api_url(&bbl));
    serde_json::from_str(&addr_info).unwrap()
}

fn get_agg_results(bbl: BBL) -> WOWAggResults {
    let agg_info = get_from_cache_or_download(format!("{}-agg.json", bbl), wow_aggregate_api_url(&bbl));
    serde_json::from_str(&agg_info).unwrap()
}

fn main() {
    let mut conh_bbls = HashSet::new();
    let mut portfolios = PortfolioBuilder::new();
    let conh_records = Vec::from_iter(iter_conh_records());
    let mut agg_results = HashMap::new();
    for rec in conh_records.iter() {
        let bbl = rec.as_bbl();
        if !conh_bbls.insert(bbl) {
            println!("Warning: BBL {} exists at least twice in CONH candidates!", bbl);
        }
    }
    for (i, rec) in conh_records.iter().enumerate() {
        let bbl = rec.as_bbl();
        let row_num = i + 1;
        if row_num % 50 == 0 || row_num == 1 {
            println!("Processing row #{}.", row_num);
        }

        agg_results.insert(bbl, get_agg_results(bbl));

        portfolios.define(&bbl);
        for addr in get_addr_results(bbl).addrs.iter() {
            portfolios.associate(&bbl, &addr.as_bbl());
        }
    }
    println!("Found {} unique CONH BBLs over {} rows.", conh_bbls.len(), conh_records.len());
    println!("Portfolios span a total of {} unique BBLs.", portfolios.num_bbls());
    let pmap = portfolios.get_portfolios();
    println!("Found {} disjoint portfolios.", pmap.portfolios.len());
    let mut writer = csv::Writer::from_path(std::path::Path::new(OUTPUT_FILENAME)).unwrap();
    println!("Writing {}...", OUTPUT_FILENAME);
    for rec in conh_records.iter() {
        let bbl = rec.as_bbl();
        let agg = &agg_results.get(&bbl).unwrap().result[0];
        let virtual_portfolio_id = *pmap.bbl_mapping.get(&bbl).unwrap();
        let virtual_portfolio_bbl_count = pmap.portfolios.get(virtual_portfolio_id).unwrap().len();
        writer.serialize(OutputRecord {
            building_id: rec.building_id,
            bin: rec.bin,
            street_address: rec.street_address.clone(),
            boro: rec.boro,
            block: rec.block,
            lot: rec.lot,
            bbl: bbl.to_string(),
            bldgs: agg.bldgs.clone(),
            units: agg.units.clone(),
            topowners: agg.topowners.as_ref().map_or(String::from(""), |v| v.join(", ")),
            topcorp: agg.topcorp.clone(),
            topbusinessaddr: agg.topbusinessaddr.clone(),
            virtual_portfolio_id,
            virtual_portfolio_bbl_count,
        }).unwrap();
        writer.flush().unwrap();
    }
    println!("Done.");
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
fn test_wow_address_api_url_works() {
    assert_eq!(wow_address_api_url(&BBL::new(1, 5099, 39)), "https://whoownswhat.justfix.nyc/api/address?block=05099&lot=0039&borough=1");
}

#[test]
fn test_wow_aggregate_api_url_works() {
    assert_eq!(wow_aggregate_api_url(&BBL::new(1, 5099, 39)), "https://whoownswhat.justfix.nyc/api/address/aggregate?bbl=1050990039");
}
