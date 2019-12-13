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

fn load_csv() -> std::io::Result<()> {
    let file = File::open("./data/Certification_of_No_Harassment__CONH__Pilot_Building_List.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut count = 0;
    for result in rdr.deserialize() {
        let record: CONHRecord = result?;
        println!("Row: {:?}", record);
        count += 1;
        if count > 10 {
            break;
        }
    }
    Ok(())
}

fn main() {
    match load_csv() {
        Ok(()) => println!("Done."),
        Err(e) => println!("Error! {:?}", e)
    }
}
