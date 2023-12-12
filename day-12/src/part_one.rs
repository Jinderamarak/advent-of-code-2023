use super::parsing::{DataEntry, State};

pub fn solve(data: &str) -> Vec<u32> {
    data.lines()
        .map(DataEntry::parse)
        .map(count_combinations)
        .collect()
}

fn count_combinations(entry: DataEntry) -> u32 {
    let unknown = entry
        .springs
        .iter()
        .filter(|s| **s == State::Unknown)
        .count();

    let mut counter = 0;
    let mut combined = entry.clone();
    for comb in create_combinations(unknown as u32) {
        combined.springs = merge(&entry.springs, &comb);
        if combined.is_valid() {
            counter += 1;
        }
    }

    counter
}

fn merge(unknown: &[State], comb: &[State]) -> Vec<State> {
    let mut iter = comb.into_iter();
    unknown
        .into_iter()
        .map(|s| {
            if *s == State::Unknown {
                *iter.next().unwrap()
            } else {
                *s
            }
        })
        .collect()
}

fn create_combinations(n: u32) -> Vec<Vec<State>> {
    let max = 2_u32.pow(n);
    (0..max)
        .map(|i| create_combination(Vec::new(), n, i))
        .collect()
}

fn create_combination(mut current: Vec<State>, remain: u32, seed: u32) -> Vec<State> {
    if remain == 0 {
        return current;
    }

    if seed & 2_u32.pow(remain - 1) > 0 {
        current.push(State::Broken);
    } else {
        current.push(State::Operational);
    }

    create_combination(current, remain - 1, seed)
}
