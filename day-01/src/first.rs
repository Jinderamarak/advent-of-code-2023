pub fn first(data: &str) -> Vec<u32> {
    data.lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>();

            let first = digits[0];
            let last = digits[digits.len() - 1];
            first * 10 + last
        })
        .collect()
}
