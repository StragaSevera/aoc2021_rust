use aoc2021::*;

pub fn result() -> String {
    let input = read_file("src/day03a/input.txt");
    let result = plot_course(input);
    format!("gamma: {}, epsilon: {}, product: {}", result.0, result.1, result.0 * result.1)
}

fn plot_course(input: impl Iterator<Item=String>) -> (i32, i32) {
    let mut numbers = input.peekable();
    let number_length = numbers.peek().expect("Not enough numbers").len();
    let mut input_length = 0;
    let mut digit_count = vec![0; number_length];
    for number in numbers {
        input_length += 1;
        for (i, char) in number.chars().enumerate() {
            match char {
                '0' => {}
                '1' => digit_count[i] += 1,
                _ => { panic!("Wrong character in binary number!") }
            }
        }
    }

    let mut gamma = String::with_capacity(number_length);
    let mut epsilon = String::with_capacity(number_length);
    let halfway = input_length / 2;
    let modulo = input_length % 2;
    for one_count in digit_count {
        if one_count == halfway && modulo == 0 { // 10 elements, 5 ones, 5 zeroes
            panic!("Both 1 and 0 have equal frequencies!")
        } else if one_count <= halfway { // 11 elements, 5 ones, 6 zeroes
            gamma.push('0');
            epsilon.push('1');
        } else { // 11 elements, 6 ones, 5 zeroes
            gamma.push('1');
            epsilon.push('0');
        }
    }
    (
        i32::from_str_radix(gamma.as_str(), 2).unwrap(),
        i32::from_str_radix(epsilon.as_str(), 2).unwrap()
    )
}

#[cfg(test)]
mod tests {
    use fluent_asserter::prelude::*;
    use indoc::indoc;
    use super::*;

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
        assert_that!(result).is_equal_to((22, 9));
    }
}