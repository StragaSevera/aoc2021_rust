fn main() {
    let input = aoc2021::read_file("src/bin/01b/input.txt");
    let result = calibrate(input);
    println!("{}", result);
}

const WINDOW_SIZE: usize = 3;

fn calibrate<S: AsRef<str>>(input: impl Iterator<Item=S>) -> i32 {
    let numbers: Vec<i32> = input.map(|line| line.as_ref().parse::<i32>().unwrap()).collect();
    if numbers.len() < WINDOW_SIZE + 1 { panic!("Insufficient input!") }
    let mut size = 0;
    let mut amount = 0;
    for i in 0..numbers.len() {
        if i < WINDOW_SIZE {
            size += numbers[i];
        } else {
            let new_size = size + numbers[i] - numbers[i - WINDOW_SIZE];
            if new_size > size { amount += 1 };
            size = new_size;
        }
    }

    amount
}

#[cfg(test)]
mod tests {
    use fluent_asserter::prelude::*;
    use indoc::indoc;
    use super::*;

    const INPUT: &str = indoc! {"
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
    "};

    #[test]
    fn calibrate_correct() {
        let result = calibrate(INPUT.lines());
        assert_that!(result).is_equal_to(5);
    }

    #[test]
    fn calibrate_small() {
        let result = || calibrate("1\n2\n3".lines());
        assert_that_code!(result).panics().with_message("Insufficient input!")
    }
}