const WORDIGITS: [(&'static str, u32); 18] = [
    ("one", 1),
    ("1", 1),
    ("two", 2),
    ("2", 2),
    ("three", 3),
    ("3", 3),
    ("four", 4),
    ("4", 4),
    ("five", 5),
    ("5", 5),
    ("six", 6),
    ("6", 6),
    ("seven", 7),
    ("7", 7),
    ("eight", 8),
    ("8", 8),
    ("nine", 9),
    ("9", 9),
];

pub fn second(data: &str) -> Vec<u32> {
    data.lines()
        .map(|l| {
            let first = first_wordigit(l);
            let last = last_wordigit(l);
            first * 10 + last
        })
        .collect()
}

fn first_wordigit(line: &str) -> u32 {
    for i in 0..line.len() {
        let s: String = line.chars().skip(i).collect();
        for (word, value) in WORDIGITS {
            if s.starts_with(word) {
                return value;
            }
        }
    }

    panic!("Line '{line}' doesn't contain any wordigits");
}

fn last_wordigit(line: &str) -> u32 {
    for i in 0..line.len() {
        let s: String = line.chars().take(line.len() - i).collect();
        for (word, value) in WORDIGITS {
            if s.ends_with(word) {
                return value;
            }
        }
    }

    panic!("Line '{line}' doesn't contain any wordigits");
}
