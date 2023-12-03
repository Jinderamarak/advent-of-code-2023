mod parsing;
mod part_one;
mod part_two;

fn main() {
    let full = utils::get_full_input(2023, 3).unwrap();

    let first_output = part_one::find_part_numbers(&full);
    let first_sum = first_output.iter().sum::<u32>();
    println!("Part one answer: {first_sum}");

    let second_output = part_two::get_gear_ratios(&full);
    let second_sum = second_output.iter().sum::<u32>();
    println!("Part two answer: {second_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = utils::get_example_input("03/first.txt").unwrap();
        let output = part_one::find_part_numbers(&input);

        let expected = vec![467, 35, 633, 617, 592, 755, 664, 598];
        assert_eq!(expected, output);
    }

    #[test]
    fn part_two_example() {
        let input = utils::get_example_input("03/first.txt").unwrap();
        let output = part_two::get_gear_ratios(&input);

        let expected = vec![16345, 451490];
        assert_eq!(expected, output);
    }
}
