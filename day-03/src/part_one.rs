pub fn find_part_numbers(data: &str) -> Vec<u32> {
    let grid: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();

    let mut numbers = Vec::new();
    for y in 0..grid.len() {

        let mut is_adjacent = false;
        let mut num_chars = Vec::new();

        for x in 0..grid[y].len() {
            let c = grid[y][x];
            if c.is_digit(10) {
                is_adjacent |= has_adjacent_symbol(&grid, x, y);
                num_chars.push(c);
            } else {
                if is_adjacent && num_chars.len() > 0 {
                    numbers.push(chars_to_num(&num_chars));
                    is_adjacent = false;
                    num_chars.clear();
                }
                is_adjacent = false;
                num_chars.clear();
            }
        }

        if is_adjacent && num_chars.len() > 0 {
            numbers.push(chars_to_num(&num_chars));
        }
    }

    numbers
}

fn has_adjacent_symbol(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    let sy = y.max(1) - 1;
    let ey = y.min(grid.len() - 2) + 1;
    let sx = x.max(1) - 1;

    for oy in sy..=ey {
        let ex = x.min(grid[oy].len() - 2) + 1;
        for ox in sx..=ex {
            let c = grid[oy][ox];
            if !c.is_digit(10) && c != '.' {
                return true;
            }
        }
    }

    false
}

fn chars_to_num(chars: &[char]) -> u32 {
    let mut sum = 0;
    for i in 0..chars.len() {
        let digit = chars[chars.len() - i - 1].to_digit(10).unwrap();
        sum += digit * 10_u32.pow(i as u32);
    }

    sum
}