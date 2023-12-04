mod parsing;
mod part_one;
mod part_two;

fn main() {
    let full = utils::get_full_input(2023, 4).unwrap();

    let first_output = part_one::get_points_per_card(&full);
    let first_sum = first_output.iter().sum::<u32>();
    println!("Part one answer: {first_sum}");

    let second_output = part_two::get_scratchcards_counts(&full);
    let second_sum = second_output.iter().sum::<u32>();
    println!("Part two answer: {second_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = utils::get_example_input("04/first.txt").unwrap();
        let output = part_one::get_points_per_card(&input);

        let expected = vec![8, 2, 2, 1, 0, 0];
        assert_eq!(expected, output);
    }

    #[test]
    fn part_two_example() {
        let input = utils::get_example_input("04/first.txt").unwrap();
        let output = part_two::get_scratchcards_counts(&input);

        let expected = vec![1, 2, 4, 8, 14, 1];
        assert_eq!(expected, output);
    }
}
