#[derive(Debug)]
pub struct HandBid {
    pub hand: Hand,
    pub bid: u32,
}

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<char>,
}

impl HandBid {
    pub fn parse(line: &str) -> HandBid {
        let parts = line.split(" ").collect::<Vec<_>>();

        let cards = parts[0].chars().collect();
        let bid = parts[1].parse().unwrap();

        HandBid {
            hand: Hand { cards },
            bid,
        }
    }
}

pub trait HandTypes1 {
    fn is_five_of_kind(&self) -> bool;
    fn is_four_of_kind(&self) -> bool;
    fn is_full_house(&self) -> bool;
    fn is_three_of_kind(&self) -> bool;
    fn is_two_pair(&self) -> bool;
    fn is_one_pair(&self) -> bool;
    fn is_high_card(&self) -> bool;
}

pub trait HandTypes2 {
    fn is_five_of_kind(&self) -> bool;
    fn is_four_of_kind(&self) -> bool;
    fn is_full_house(&self) -> bool;
    fn is_three_of_kind(&self) -> bool;
    fn is_two_pair(&self) -> bool;
    fn is_one_pair(&self) -> bool;
    fn is_high_card(&self) -> bool;
}
