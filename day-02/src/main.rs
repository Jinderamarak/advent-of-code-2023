fn main() {
    let full = utils::get_full_input(2023, 2).unwrap();

    let first = part_one(&full).iter().sum::<u32>();

    let second = part_two(&full).iter().sum::<u32>();

    println!("{first:?}");
    println!("{second:?}");
}

#[derive(Debug)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    fn zero() -> Cubes {
        Cubes {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn parse(data: &str) -> Cubes {
        let mut result = Cubes {
            red: 0,
            green: 0,
            blue: 0,
        };

        for cube in data.split(", ") {
            let parts = cube.split(" ").collect::<Vec<_>>();
            let count = parts[0].parse::<u32>().unwrap();
            let color = parts[1];

            match color {
                "red" => result.red += count,
                "green" => result.green += count,
                "blue" => result.blue += count,
                _ => {}
            }
        }

        result
    }

    fn fits_within(&self, rules: &Cubes) -> bool {
        self.red <= rules.red && self.green <= rules.green && self.blue <= rules.blue
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    peeks: Vec<Cubes>,
}

impl Game {
    fn parse(data: &str) -> Game {
        let parts = data.split(": ").collect::<Vec<_>>();
        let id = parts[0].replace("Game ", "").parse::<u32>().unwrap();
        let peeks = parts[1].split("; ").map(Cubes::parse).collect::<Vec<_>>();
        Game { id, peeks }
    }
}

const ALL_CUBES: Cubes = Cubes {
    red: 12,
    green: 13,
    blue: 14,
};

fn part_one(data: &str) -> Vec<u32> {
    let games = data.lines().map(Game::parse).collect::<Vec<_>>();

    let mut possible_ids = Vec::new();
    for game in games {
        let possible = game.peeks.iter().all(|peek| peek.fits_within(&ALL_CUBES));
        if possible {
            possible_ids.push(game.id);
        }
    }

    possible_ids
}

fn part_two(data: &str) -> Vec<u32> {
    let games = data.lines().map(Game::parse).collect::<Vec<_>>();
    games
        .iter()
        .map(|game| {
            let init = Cubes::zero();
            let min = game.peeks.iter().fold(init, |acc, x| Cubes {
                red: acc.red.max(x.red),
                green: acc.green.max(x.green),
                blue: acc.blue.max(x.blue),
            });

            min.red * min.green * min.blue
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        let input = utils::get_example_input("02/first.txt").unwrap();
        let output = part_one(&input);

        let expected = vec![1, 2, 5];
        assert_eq!(expected, output);
    }

    #[test]
    fn second_example() {
        let input = utils::get_example_input("02/first.txt").unwrap();
        let output = part_two(&input);

        let expected = vec![48, 12, 1560, 630, 36];
        assert_eq!(expected, output);
    }
}
