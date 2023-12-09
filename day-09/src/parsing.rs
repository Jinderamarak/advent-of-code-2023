pub fn parse(data: &str) -> Vec<Vec<i64>> {
    data.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split(" ").filter_map(|n| n.parse().ok()).collect()
}
