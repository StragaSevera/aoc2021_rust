pub fn result() -> String {
    let input = aoc2021::read_file("src/day01a/input.txt");
    let result = calibrate(input);
    format!("{}", result)
}

fn calibrate<S: AsRef<str>>(input: impl Iterator<Item=S>) -> i32 {
    enum CalibrationAcc {
        Empty,
        Full(i32, i32),
    }

    let numbers = input.map(|line| line.as_ref().parse::<i32>().expect("Cannot parse number!"));
    let result = numbers.fold(CalibrationAcc::Empty, |acc, current| {
        match acc {
            CalibrationAcc::Empty => CalibrationAcc::Full(current, 0),
            CalibrationAcc::Full(old, amount) => CalibrationAcc::Full(
                current,
                if current > old { amount + 1 } else { amount },
            )
        }
    });
    match result {
        CalibrationAcc::Empty => { panic!("Empty input!") }
        CalibrationAcc::Full(_, amount) => { amount }
    }
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
        assert_that!(result).is_equal_to(7);
    }

    #[test]
    fn calibrate_empty() {
        let result = || calibrate("".lines());
        assert_that_code!(result).panics().with_message("Empty input!")
    }

    #[test]
    fn calibrate_non_number() {
        let result = || calibrate("1\na\n3\n4".lines());
        assert_that_code!(result).panics().with_having_message("Cannot parse number!")
    }
}