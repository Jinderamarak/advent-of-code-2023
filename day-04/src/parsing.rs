#[derive(Debug, Clone)]
pub struct ScratchCard {
    pub winning: Vec<u32>,
    pub numbers: Vec<u32>,
}

impl ScratchCard {
    pub fn parse(line: &str) -> ScratchCard {
        let data = line.split(": ").nth(1).unwrap();
        let parts: Vec<&str> = data.split(" | ").collect();

        let winning = Self::get_num_list(parts[0]);
        let numbers = Self::get_num_list(parts[1]);
        ScratchCard {
            winning, numbers
        }
    }

    fn get_num_list(list: &str) -> Vec<u32> {
        list.split(" ").filter_map(|i| i.parse().ok()).collect()
    }
}
