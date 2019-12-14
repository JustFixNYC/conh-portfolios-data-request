use std::fs::File;
use serde::Deserialize;

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
    fn bbl(&self) -> String {
        format!("{}{:05}{:04}", self.boro, self.block, self.lot)
    }

    fn wow_address_api_url(&self) -> String {
        format!("https://whoownswhat.justfix.nyc/api/address?block={:05}&lot={:04}&borough={}", self.block, self.lot, self.boro)
    }
}

fn download_wow_info(rec: &CONHRecord) -> String {
    let url = rec.wow_address_api_url();
    let result = reqwest::get(url.as_str()).unwrap().text().unwrap();
    result
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
        println!("WOW info: {}", download_wow_info(&rec));
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
