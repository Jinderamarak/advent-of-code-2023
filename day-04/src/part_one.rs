use super::parsing::ScratchCard;

pub fn get_points_per_card(data: &str) -> Vec<u32> {
    data.lines().map(ScratchCard::parse).map(count_score).collect()
}

fn count_score(card: ScratchCard) -> u32 {
    card.numbers.iter().fold(0, |acc, i| {
        if card.winning.contains(i) {
            if acc == 0 {
                1
            } else {
                acc * 2
            }
        } else {
            acc
        }
    })
}
