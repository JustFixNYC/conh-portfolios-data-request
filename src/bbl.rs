#[derive(PartialEq)]
pub struct BBL {
    boro: u8,
    block: u32,
    lot: u16,
}

impl BBL {
    pub fn new(boro: u8, block: u32, lot: u16) -> BBL {
        BBL { boro, block, lot }
    }
}

impl std::fmt::Display for BBL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:05}{:04}", self.boro, self.block, self.lot)
    }
}

#[test]
fn test_to_string_works() {
    assert_eq!(BBL::new(1, 5099, 39).to_string(), "1050990039");
}
