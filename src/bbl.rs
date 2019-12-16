use std::str::FromStr;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct BBL {
    pub boro: u8,
    pub block: u32,
    pub lot: u16,
}

#[derive(Debug, PartialEq)]
pub enum BBLParseError {
    InvalidInt,
    InvalidLength,
}

impl BBL {
    pub fn new(boro: u8, block: u32, lot: u16) -> BBL {
        BBL { boro, block, lot }
    }
}

impl PartialOrd for BBL {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BBL {
    fn cmp(&self, other: &Self) -> Ordering {
        let boro = self.boro.cmp(&other.boro);
        if boro == Ordering::Equal {
            let block = self.block.cmp(&other.block);
            if block == Ordering::Equal {
                self.lot.cmp(&other.lot)
            } else {
                block
            }
        } else {
            boro
        }
    }
}

impl std::fmt::Display for BBL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:05}{:04}", self.boro, self.block, self.lot)
    }
}

impl FromStr for BBL {
    type Err = BBLParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 10 {
            Err(BBLParseError::InvalidLength)
        } else {
            match (u8::from_str(&s[0..1]), u32::from_str(&s[1..6]), u16::from_str(&s[6..10])) {
                (Ok(boro), Ok(block), Ok(lot)) => Ok(BBL::new(boro, block, lot)),
                _ => Err(BBLParseError::InvalidInt),
            }
        }
    }
}

#[test]
fn test_to_string_works() {
    assert_eq!(BBL::new(1, 5099, 39).to_string(), "1050990039");
}

#[test]
fn test_from_str_raises_invalid_length_err() {
    assert_eq!(BBL::from_str("1"), Err(BBLParseError::InvalidLength));
}

#[test]
fn test_from_str_raises_parse_int_err() {
    let err = Err(BBLParseError::InvalidInt);
    assert_eq!(BBL::from_str("a234567890"), err);
    assert_eq!(BBL::from_str("1a34567890"), err);
    assert_eq!(BBL::from_str("123456789a"), err);
}

#[test]
fn test_from_str_works() {
    assert_eq!(BBL::from_str("1050990039"), Ok(BBL::new(1, 5099, 39)));
}

#[test]
fn test_ord_works() {
    let bbl = BBL::new(1, 5099, 39);
    assert!(BBL::new(2, 5099, 39) > bbl); // Boro is greater
    assert!(BBL::new(1, 5100, 39) > bbl); // Block is greater
    assert!(BBL::new(1, 5099, 40) > bbl); // Lot is greater
    assert_eq!(BBL::new(1, 5099, 39).cmp(&bbl), Ordering::Equal);
}
