use aoc2021::*;
use std::collections::BTreeSet;

pub fn result() -> String {
    let input = read_file("src/day03b/input.txt");
    let result = plot_course(input);
    format!(
        "O2: {}, CO2: {}, product: {}",
        result.0,
        result.1,
        result.0 * result.1
    )
}

fn plot_course(input: impl Iterator<Item = String>) -> (i32, i32) {
    let o2_numbers: BTreeSet<Vec<char>> = input.map(|s| s.chars().collect()).collect();
    let co2_numbers = o2_numbers.clone();

    let o2_result = find_value(o2_numbers, |zero_numbers, one_numbers| {
        if zero_numbers.len() > one_numbers.len() {
            zero_numbers
        } else {
            one_numbers
        }
    });
    let co2_result = find_value(co2_numbers, |zero_numbers, one_numbers| {
        if zero_numbers.len() <= one_numbers.len() {
            zero_numbers
        } else {
            one_numbers
        }
    });

    (o2_result, co2_result)
}

fn find_value(
    mut numbers: BTreeSet<Vec<char>>,
    choose_numbers: impl Fn(BTreeSet<Vec<char>>, BTreeSet<Vec<char>>) -> BTreeSet<Vec<char>>,
) -> i32 {
    let number_length = numbers.iter().next().unwrap().len();
    for i in 0..number_length {
        if numbers.len() == 1 {
            break;
        }
        // BTreeSet iterates its elements in ascending order, so all 0 will come first before 1
        let first_one = numbers.iter().find(|s| s[i] == '1').cloned();
        if let Some(ref first_one) = first_one {
            let mut zero_numbers = numbers;
            let one_numbers = zero_numbers.split_off(first_one);
            numbers = choose_numbers(zero_numbers, one_numbers);
        }
    }
    assert_eq!(numbers.len(), 1);
    let result = numbers.iter().next().unwrap().iter().collect::<String>();
    i32::from_str_radix(&result, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use fluent_asserter::prelude::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    "};

    #[test]
    fn calibrate_correct() {
        let result = plot_course(read_string(INPUT));
        assert_that!(result).is_equal_to((23, 10));
    }
}
