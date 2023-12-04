use super::parsing::ScratchCard;

pub fn get_scratchcards_count(data: &str) -> Vec<u32> {
    let matches: Vec<u32> = data.lines().map(ScratchCard::parse).map(count_matches).collect();
    let mut counts = vec![1; matches.len()];

    for i in 0..matches.len() {
        let c = counts[i];
        let m = matches[i];

        for j in 0..m {
            counts[i + j as usize + 1] += c;
        }
    }

    counts
}

fn count_matches(card: ScratchCard) -> u32 {
    card.numbers.iter().fold(0, |acc, i| {
        if card.winning.contains(i) {
            acc + 1
        } else {
            acc
        }
    })
}
