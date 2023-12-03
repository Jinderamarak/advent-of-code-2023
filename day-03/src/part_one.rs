use super::parsing::StrNum;

pub fn find_part_numbers(data: &str) -> Vec<u32> {
    let symbols: Vec<(usize, usize)> = data
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| !c.is_digit(10) && *c != '.')
                .map(move |(x, _)| (x, y))
        })
        .flatten()
        .collect();

    data.lines()
        .enumerate()
        .map(StrNum::parse)
        .flatten()
        .filter(|n| symbols.iter().any(|(x, y)| n.is_adjacent(*x, *y)))
        .map(|n| n.value)
        .collect()
}
