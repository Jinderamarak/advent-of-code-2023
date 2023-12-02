mod parsing;
mod part_one;
mod part_two;

fn main() {
    let full = utils::get_full_input(2023, 2).unwrap();

    let first_output = part_one::get_possible_ids(&full);
    let first_sum = first_output.iter().sum::<u32>();
    println!("Part one answer: {first_sum}");

    let second_output = part_two::get_game_powers(&full);
    let second_sum = second_output.iter().sum::<u32>();
    println!("Part two answer: {second_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = utils::get_example_input("02/first.txt").unwrap();
        let output = part_one::get_possible_ids(&input);

        let expected = vec![1, 2, 5];
        assert_eq!(expected, output);
    }

    #[test]
    fn part_two_example() {
        let input = utils::get_example_input("02/first.txt").unwrap();
        let output = part_two::get_game_powers(&input);

        let expected = vec![48, 12, 1560, 630, 36];
        assert_eq!(expected, output);
    }
}
