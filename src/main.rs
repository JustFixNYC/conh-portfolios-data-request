use std::fs::File;
use serde::{Deserialize, Deserializer};

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

#[derive(Debug, Deserialize)]
struct WOWAddrResult {
    bbl: String,
}

#[derive(Debug, Deserialize)]
struct WOWAddrResults {
    addrs: Vec<WOWAddrResult>,
}

#[derive(Debug, Deserialize)]
struct WOWAggResult {
    #[serde(deserialize_with = "from_str")]
    bldgs: u32,

    #[serde(deserialize_with = "from_str")]
    units: u32,

    topowners: Vec<String>,
    topcorp: String,
    topbusinessaddr: String,

    #[serde(deserialize_with = "from_str")]
    totalopenviolations: u32,

    #[serde(deserialize_with = "from_str")]
    totalviolations: u32,
}

#[derive(Debug, Deserialize)]
struct WOWAggResults {
    result: Vec<WOWAggResult>,
}

impl CONHRecord {
    fn bbl(&self) -> String {
        format!("{}{:05}{:04}", self.boro, self.block, self.lot)
    }

    fn wow_address_api_url(&self) -> String {
        format!("{}/address?block={:05}&lot={:04}&borough={}", WOW_API_ROOT, self.block, self.lot, self.boro)
    }

    fn wow_aggregate_api_url(&self) -> String {
        format!("{}/address/aggregate?bbl={}", WOW_API_ROOT, self.bbl())
    }
}

// https://github.com/serde-rs/json/issues/317#issuecomment-300251188
fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
  where T: std::str::FromStr,
        T::Err: std::fmt::Display,
        D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}

fn get_from_cache_or_download(filename: String, url: String) -> String {
    let path_str = format!("./data/wow/{}", filename);
    let path = std::path::Path::new(&path_str);
    if !path.exists() {
        let cache_dir = path.parent().unwrap();
        if !cache_dir.exists() {
            std::fs::create_dir_all(cache_dir).unwrap();
        }
        let result = reqwest::get(&url).unwrap().text().unwrap();
        std::fs::write(path, result).unwrap();
    }
    std::fs::read_to_string(path).unwrap()
}

fn iter_conh_records() -> impl Iterator<Item = CONHRecord> {
    let file = File::open("./data/Certification_of_No_Harassment__CONH__Pilot_Building_List.csv").unwrap();
    let rdr = csv::Reader::from_reader(file);
    let records = rdr.into_deserialize::<CONHRecord>();
    records.map(|rec| rec.unwrap())
}

fn main() {
    for (i, rec) in iter_conh_records().enumerate() {
        if i > 1 { break; }
        println!("Row #{} {}", i + 1, rec.bbl());
        let addr_info = get_from_cache_or_download(format!("{}-addr.json", rec.bbl()), rec.wow_address_api_url());
        let agg_info = get_from_cache_or_download(format!("{}-agg.json", rec.bbl()), rec.wow_aggregate_api_url());
        println!("  WOW addr info: {} bytes, agg info: {} bytes", addr_info.len(), agg_info.len());
        let addr_results: WOWAddrResults = serde_json::from_str(&addr_info).unwrap();
        for addr in addr_results.addrs.iter() {
            println!("  BBL in portfolio: {}", addr.bbl);
        }
        let agg_results: WOWAggResults = serde_json::from_str(&agg_info).unwrap();
        for result in agg_results.result.iter() {
            println!("  Units in portfolio: {}", result.units);
        }
    }
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
    assert_eq!(rec.bbl(), "1050990039");
}

#[test]
fn test_wow_address_api_url_works() {
    let rec = CONHRecord { boro: 1, block: 5099, lot: 39 };
    assert_eq!(rec.wow_address_api_url(), "https://whoownswhat.justfix.nyc/api/address?block=05099&lot=0039&borough=1");
}

#[test]
fn test_wow_aggregate_api_url_works() {
    let rec = CONHRecord { boro: 1, block: 5099, lot: 39 };
    assert_eq!(rec.wow_aggregate_api_url(), "https://whoownswhat.justfix.nyc/api/address/aggregate?bbl=1050990039");
}
