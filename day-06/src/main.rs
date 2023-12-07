mod parsing;
mod part_one;
mod part_two;

fn main() {
    let full = utils::get_full_input(2023, 6).unwrap();

    let first_output = part_one::solve(&full);
    let first_product = first_output.iter().product::<u32>();
    println!("Part one answer: {first_product}");

    let second_output = part_two::solve(&full);
    println!("Part two answer: {second_output}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = utils::get_example_input("06/first.txt").unwrap();
        let output = part_one::solve(&input);

        let expected = vec![4, 8, 9];
        assert_eq!(expected, output);
    }

    #[test]
    fn part_two_example() {
        let input = utils::get_example_input("06/first.txt").unwrap();
        let output = part_two::solve(&input);

        assert_eq!(71503, output);
    }
}
