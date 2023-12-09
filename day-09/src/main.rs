mod parsing;
mod part_one;
mod part_two;

fn main() {
    let full = utils::get_full_input(2023, 9).unwrap();

    let first_output = part_one::solve(&full);
    println!("Part one answer: {first_output}");
    /*
    let second_output = part_two::solve(&full);
    println!("Part two answer: {second_output}");
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = utils::get_example_input("09/first.txt").unwrap();
        let output = part_one::solve(&input);

        assert_eq!(2, output);
    }
    /*
    #[test]
    fn part_two_example() {
        let input = utils::get_example_input("09/first.txt").unwrap();
        let output = part_two::solve(&input);

        assert_eq!(6, output);
    }
    */
}
