mod parsing;
mod part_one;
mod part_two;

fn main() {
    let full = utils::get_full_input(2023, 5).unwrap();

    let first_output = part_one::seeds_to_locations(&full);
    let first_min = first_output.iter().min().unwrap();
    println!("Part one answer: {first_min}");

    let second_output = part_two::seeds_to_locations(&full);
    let second_min = second_output.iter().min().unwrap();
    println!("Part two answer: {second_min}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = utils::get_example_input("05/first.txt").unwrap();
        let output = part_one::seeds_to_locations(&input);

        let expected = vec![82, 43, 86, 35];
        assert_eq!(expected, output);
    }

    #[test]
    fn part_two_example() {
        let input = utils::get_example_input("05/first.txt").unwrap();
        let output = part_two::seeds_to_locations(&input);

        assert_eq!(27, output.len());
        assert_eq!(46, *output.iter().min().unwrap());
    }
}
