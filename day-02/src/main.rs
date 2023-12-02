use parsing::{Cubes, Game};

mod parsing;

fn main() {
    let full = utils::get_full_input(2023, 2).unwrap();

    let first = part_one(&full).iter().sum::<u32>();

    let second = part_two(&full).iter().sum::<u32>();

    println!("{first:?}");
    println!("{second:?}");
}

const AVAILABLE_CUBES: Cubes = Cubes {
    red: 12,
    green: 13,
    blue: 14,
};

fn part_one(data: &str) -> Vec<u32> {
    data.lines()
        .map(Game::parse)
        .filter(|game| {
            game.peeks
                .iter()
                .all(|peek| peek.fits_within(&AVAILABLE_CUBES))
        })
        .map(|game| game.id)
        .collect()
}

fn part_two(data: &str) -> Vec<u32> {
    data.lines()
        .map(Game::parse)
        .map(|game| {
            let max = game.peeks.iter().fold(Cubes::zero(), |a, b| a.max(b));
            max.red * max.green * max.blue
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
