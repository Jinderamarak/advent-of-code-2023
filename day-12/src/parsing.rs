use core::panic;

#[derive(Debug, Clone)]
pub struct DataEntry {
    pub springs: Vec<State>,
    pub damaged: Vec<u32>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Operational,
    Broken,
    Unknown,
}

impl DataEntry {
    pub fn parse(line: &str) -> DataEntry {
        let parts = line.split(" ").collect::<Vec<_>>();
        let springs = parts[0].chars().map(State::parse).collect();
        let damaged = parts[1]
            .split(",")
            .filter_map(|n| n.parse::<u32>().ok())
            .collect();

        DataEntry { springs, damaged }
    }

    pub fn print(&self) {
        for state in &self.springs {
            print!(
                "{}",
                match state {
                    State::Operational => ".",
                    State::Broken => "#",
                    State::Unknown => "?",
                }
            );
        }

        println!(" {:?}", self.damaged);
    }

    pub fn is_valid(&self) -> bool {
        if self.springs.iter().any(|s| *s == State::Unknown) {
            return false;
        }

        let groups = self
            .springs
            .split(|s| *s == State::Operational)
            .filter(|p| p.len() > 0)
            .collect::<Vec<_>>();

        if groups.len() != self.damaged.len() {
            return false;
        }

        groups
            .iter()
            .zip(self.damaged.iter())
            .all(|(s, n)| s.len() == *n as usize)
    }
}

impl State {
    pub fn parse(c: char) -> State {
        match c {
            '.' => State::Operational,
            '#' => State::Broken,
            '?' => State::Unknown,
            _ => panic!("Unknown spring state: {c}"),
        }
    }
}
