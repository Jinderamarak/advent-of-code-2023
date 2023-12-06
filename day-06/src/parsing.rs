#[derive(Debug)]
pub struct Race {
    pub time: u64,
    pub distance: u64,
}

impl Race {
    pub fn parse(content: &str) -> Vec<Race> {
        let lines: Vec<_> = content.lines().collect();

        let times = lines[0].split(" ").filter_map(|s| s.parse().ok());
        let distances = lines[1].split(" ").filter_map(|s| s.parse().ok());

        times
            .zip(distances)
            .map(|(time, distance)| Race { time, distance })
            .collect()
    }

    pub fn parse_kerning(content: &str) -> Race {
        let lines: Vec<_> = content.lines().collect();

        let time = lines[0]
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap();

        let distance = lines[1]
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap();

        Race { time, distance }
    }

    pub fn traveled_distance(hold: u64, limit: u64) -> u64 {
        let time = limit - hold;
        time * hold
    }
}
