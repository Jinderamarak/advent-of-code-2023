mod parsing;
mod part_one;
mod part_two;

fn main() {
    let full = utils::get_full_input(2023, 3).unwrap();

    let first_output = part_one::solve(&full);
    let first_sum = first_output.iter().sum::<u32>();
    println!("Part one answer: {first_sum}");

    let second_output = part_two::solve(&full);
    let second_sum = second_output.iter().sum::<u32>();
    println!("Part two answer: {second_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = utils::get_example_input("03/first.txt").unwrap();
        let output = part_one::solve(&input);

        let expected = vec![1, 2, 3];
        assert_eq!(expected, output);
    }

    #[test]
    fn part_two_example() {
        let input = utils::get_example_input("03/second.txt").unwrap();
        let output = part_two::solve(&input);

        let expected = vec![1, 2, 3];
        assert_eq!(expected, output);
    }
}
