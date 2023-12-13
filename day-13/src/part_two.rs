use super::shared::{Pattern, Reflection};

pub fn solve(data: &str) -> Vec<u32> {
    data.split("\n\n")
        .map(Pattern::parse)
        .map(|p| p.find_reflection(&is_reflection_line))
        .map(Reflection::summarize)
        .collect()
}

fn is_reflection_line(pat: &Pattern, y: usize) -> bool {
    let mut smudges = 0;
    for i in 0..y.min(pat.h - y) {
        let y1 = y + i;
        let y2 = y - i - 1;
        smudges += pat.rows[y1]
            .iter()
            .zip(pat.rows[y2].iter())
            .filter(|(a, b)| a != b)
            .count();

        if smudges > 1 {
            return false;
        }
    }

    smudges == 1
}
