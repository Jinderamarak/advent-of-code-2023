use parsing::{Cubes, Game};

mod parsing;

fn main() {
    let full = utils::get_full_input(2023, 2).unwrap();

    let first_output = part_one(&full);
    let first_sum = first_output.iter().sum::<u32>();
    println!("Part one answer: {first_sum}");

    let second_output = part_two(&full);
    let second_sum = second_output.iter().sum::<u32>();
    println!("Part two answer: {second_sum}");
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
    fn part_one_example() {
        let input = utils::get_example_input("02/first.txt").unwrap();
        let output = part_one(&input);

        let expected = vec![1, 2, 5];
        assert_eq!(expected, output);
    }

    #[test]
    fn part_two_example() {
        let input = utils::get_example_input("02/first.txt").unwrap();
        let output = part_two(&input);

        let expected = vec![48, 12, 1560, 630, 36];
        assert_eq!(expected, output);
    }
}
