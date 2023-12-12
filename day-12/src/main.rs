mod parsing;
mod part_one;
mod part_two;

fn main() {
    let full = utils::get_full_input(2023, 12).unwrap();

    let first_output = part_one::solve(&full);
    let first_sum = first_output.iter().sum::<u32>();
    println!("Part one answer: {first_sum}");

    //let second_output = part_two::solve(&full);
    //let second_sum = second_output.iter().sum::<usize>();
    //println!("Part two answer: {second_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = utils::get_example_input("12/first.txt").unwrap();
        let output = part_one::solve(&input);

        let expected = vec![1, 4, 1, 1, 4, 10];
        assert_eq!(expected, output);
    }

    #[test]
    fn part_two_example() {
        let input = utils::get_example_input("12/first.txt").unwrap();
        //let output = part_two::solve(&input);

        let expected = vec![1, 16384, 1, 16, 2500, 506250];
        //assert_eq!(expected, output);
    }
}
