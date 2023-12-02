mod part_one;
mod part_two;

fn main() {
    let full = utils::get_full_input(2023, 1).unwrap();

    let first_output = part_one::get_values(&full);
    let first_sum = first_output.iter().sum::<u32>();
    println!("Part one answer: {first_sum}");

    let second_output = part_two::get_values(&full);
    let second_sum = second_output.iter().sum::<u32>();
    println!("Part two answer: {second_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = utils::get_example_input("01/first.txt").unwrap();
        let output = part_one::get_values(&input);

        let expected = vec![12, 38, 15, 77];
        assert_eq!(expected, output);
    }

    #[test]
    fn part_two_example() {
        let input = utils::get_example_input("01/second.txt").unwrap();
        let output = part_two::get_values(&input);

        let expected = vec![29, 83, 13, 24, 42, 14, 76];
        assert_eq!(expected, output);
    }
}
