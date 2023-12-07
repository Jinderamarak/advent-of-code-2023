#[derive(Debug)]
pub struct HandBid {
    pub hand: Hand,
    pub bid: u32,
}

impl HandBid {
    pub fn parse(line: &str) -> HandBid {
        todo!();
    }
}
