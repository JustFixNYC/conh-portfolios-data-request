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

fn iter_conh_records() -> impl Iterator<Item = CONHRecord> {
    let file = File::open("./data/Certification_of_No_Harassment__CONH__Pilot_Building_List.csv").unwrap();
    let rdr = csv::Reader::from_reader(file);
    let records = rdr.into_deserialize::<CONHRecord>();
    records.map(|rec| rec.unwrap())
}

fn main() {
    for (i, rec) in iter_conh_records().enumerate() {
        if i > 10 { break; }
        println!("Row #{} {:?}", i + 1, rec);
    }
}
