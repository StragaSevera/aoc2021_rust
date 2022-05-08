pub fn result() -> String {
    let input = aoc2021::read_file("src/day02a/input.txt");
    let result = calibrate(input);
    let product = result.x * result.y;
    format!("x: {}, y: {}, product: {}", result.x, result.y, product)
}

#[derive(PartialEq, Eq)]
struct Coords {
    x: i32,
    y: i32,
}

fn calibrate<S: AsRef<str>>(input: impl Iterator<Item=S>) -> Coords {
    let numbers = input.map(|line| line.as_ref().parse::<i32>().expect("Cannot parse number!"));
    todo!()
}

#[cfg(test)]
mod tests {
    use fluent_asserter::prelude::*;
    use indoc::indoc;
    use super::*;

    const INPUT: &str = indoc! {"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    "};

    #[test]
    fn calibrate_correct() {
        let result = calibrate(INPUT.lines());
        assert_that!(result).is_equal_to(Coords {x = 15, y = 10});
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