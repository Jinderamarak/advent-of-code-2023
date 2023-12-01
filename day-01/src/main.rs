mod first;
mod second;

use first::first;
use second::second;

fn main() {
    let full = utils::get_full_input(2023, 1).unwrap();

    let first_output = first(&full);
    let first_sum = first_output.iter().sum::<u32>();
    println!("First answer: {first_sum}");

    let second_output = second(&full);
    let second_sum = second_output.iter().sum::<u32>();
    println!("Second answer: {second_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use first::first;
    use second::second;

    #[test]
    fn first_example() {
        let input = utils::get_example_input("01/first.txt").unwrap();
        let output = first(&input);

        let expected = vec![12, 38, 15, 77];
        assert_eq!(expected, output);
    }

    #[test]
    fn second_example() {
        let input = utils::get_example_input("01/second.txt").unwrap();
        let output = second(&input);

        let expected = vec![29, 83, 13, 24, 42, 14, 76];
        assert_eq!(expected, output);
    }
}
