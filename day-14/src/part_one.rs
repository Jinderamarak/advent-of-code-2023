use super::shared::{Platform, Tile};

pub fn solve(data: &str) -> u64 {
    let mut platform = Platform::parse(data);
    tilt_north(&mut platform);
    calculate_load(&platform)
}

fn calculate_load(platform: &Platform) -> u64 {
    let mut sum = 0;
    for y in 0..platform.h {
        let n: u64 = (platform.h - y).try_into().unwrap();
        for x in 0..platform.w {
            if platform.rows[y][x] == Tile::Rock {
                sum += n;
            }
        }
    }

    sum
}

fn tilt_north(platform: &mut Platform) {
    for y in 0..platform.h {
        for x in 0..platform.w {
            if platform.rows[y][x] == Tile::Rock {
                move_north(platform, x, y);
            }
        }
    }
}

fn move_north(p: &mut Platform, x: usize, y: usize) {
    p.rows[y][x] = Tile::Air;
    for n in (0..=y).rev() {
        if p.rows[n][x] != Tile::Air {
            p.rows[n + 1][x] = Tile::Rock;
            return;
        }
    }
    p.rows[0][x] = Tile::Rock;
}
