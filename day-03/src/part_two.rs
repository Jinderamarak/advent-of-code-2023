pub fn get_gear_ratios(data: &str) -> Vec<u32> {
    let grid: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();
    let numbers: Vec<StrNum> = data.lines().enumerate().map(|(i, l)| StrNum::parse(l, i)).flatten().collect();

    let mut ratios = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '*' {
                let adjacent: Vec<&StrNum> = numbers.iter().filter(|n| n.is_adjacent(x, y)).collect();
                if adjacent.len() == 2 {
                    ratios.push(adjacent[0].value * adjacent[1].value);
                }
            }
        }
    }


    ratios
}

#[derive(Debug)]
struct StrNum {
    value: u32,
    start: usize,
    end: usize,
    y: usize,
}

impl StrNum {
    fn single(start: usize, y: usize, nums: &[u32]) -> StrNum {
        let mut value = 0;
        for i in 0..nums.len() {
            let digit = nums[nums.len() - i - 1];
            value += digit * 10_u32.pow(i as u32);
        }

        StrNum {
            value, start, y,
            end: start + nums.len() - 1
        }
    }

    fn parse(line: &str, y: usize) -> Vec<StrNum> {
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

    fn is_adjacent(&self, x: usize, y: usize) -> bool {
        self.y <= y + 1 && self.y + 1 >= y && self.start <= x + 1 && self.end + 1 >= x
    }
}