mod part_one;
mod part_two;
mod shared;

fn main() {
    let full = utils::get_full_input(2023, 11).unwrap();

    let first_output = part_one::solve(&full);
    let first_sum = first_output.iter().sum::<usize>();
    println!("Part one answer: {first_sum}");

    let second_output = part_two::solve(&full);
    let second_sum = second_output.iter().sum::<usize>();
    println!("Part two answer: {second_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = utils::get_example_input("11/first.txt").unwrap();
        let output = part_one::solve(&input);

        assert_eq!(36, output.len());
        assert_eq!(374, output.iter().sum::<usize>());

        let expected = vec![9, 15, 17, 5];
        for e in expected {
            assert!(output.contains(&e));
        }
    }

    #[test]
    fn part_two_example() {
        let input = utils::get_example_input("11/first.txt").unwrap();
        let output = part_two::solve(&input);

        assert_eq!(36, output.len());
        assert_eq!(82000210, output.iter().sum::<usize>());
    }
}
