use super::parsing::Pattern;

pub fn solve(data: &str) -> Vec<u32> {
    data.split("\n\n")
        .map(Pattern::parse)
        .map(find_reflection)
        .collect()
}

fn rotate_pattern(pattern: &Pattern) -> Pattern {
    let mut rows: Vec<Vec<char>> = (0..pattern.w)
        .map(|_| (0..pattern.h).map(|_| 'x').collect())
        .collect();
    for y in 0..pattern.h {
        for x in 0..pattern.w {
            rows[x][y] = pattern.rows[y][x];
        }
    }

    Pattern {
        w: pattern.h,
        h: pattern.w,
        rows,
    }
}

fn find_reflection(pattern: Pattern) -> u32 {
    if let Some(a) = find_horizontal(&pattern) {
        return a * 100;
    }

    let rot = rotate_pattern(&pattern);
    if let Some(x) = find_horizontal(&rot) {
        x
    } else {
        println!();
        for row in pattern.rows {
            for c in row {
                print!("{c}");
            }
            println!();
        }

        panic!();
    }
}

fn find_horizontal(pat: &Pattern) -> Option<u32> {
    for y in 1..pat.h {
        if is_reflection_line(pat, y) {
            return Some(y.try_into().unwrap());
        }
    }

    None
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
            return false
        }
    }

    smudges == 1
}
