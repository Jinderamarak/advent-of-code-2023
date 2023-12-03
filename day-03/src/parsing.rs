#[derive(Debug)]
pub struct StrNum {
    pub value: u32,
    pub start: usize,
    pub end: usize,
    pub y: usize,
}

impl StrNum {
    fn single(start: usize, y: usize, nums: &[u32]) -> StrNum {
        let mut value = 0;
        for i in 0..nums.len() {
            let digit = nums[nums.len() - i - 1];
            value += digit * 10_u32.pow(i as u32);
        }

        StrNum {
            value,
            start,
            y,
            end: start + nums.len() - 1,
        }
    }

    pub fn parse(item: (usize, &str)) -> Vec<StrNum> {
        let (y, line) = item;

        let chars = line.chars().collect::<Vec<char>>();
        let mut results = Vec::new();

        let mut start = 0;
        let mut nums = Vec::new();
        for i in 0..chars.len() {
            if chars[i].is_digit(10) {
                if nums.len() == 0 {
                    start = i;
                }
                nums.push(chars[i].to_digit(10).unwrap());
            } else {
                if (nums.len() > 0) {
                    results.push(StrNum::single(start, y, &nums));
                }
                nums.clear();
            }
        }

        if nums.len() > 0 {
            results.push(StrNum::single(start, y, &nums));
        }

        results
    }

    pub fn is_adjacent(&self, x: usize, y: usize) -> bool {
        self.y <= y + 1 && self.y + 1 >= y && self.start <= x + 1 && self.end + 1 >= x
    }
}
