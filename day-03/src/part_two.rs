use super::parsing::StrNum;

pub fn get_gear_ratios(data: &str) -> Vec<u32> {
    let numbers: Vec<StrNum> = data
        .lines()
        .enumerate()
        .map(StrNum::parse)
        .flatten()
        .collect();

    data.lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '*')
                .map(move |(x, _)| (x, y))
        })
        .flatten()
        .filter_map(|(x, y)| try_get_gear_ratio(x, y, &numbers))
        .collect()
}

fn try_get_gear_ratio(x: usize, y: usize, numbers: &[StrNum]) -> Option<u32> {
    let adjacent: Vec<&StrNum> = numbers.iter().filter(|n| n.is_adjacent(x, y)).collect();
    if adjacent.len() == 2 {
        Some(adjacent[0].value * adjacent[1].value)
    } else {
        None
    }
}
