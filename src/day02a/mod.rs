pub fn result() -> String {
    let input = aoc2021::read_file("src/day02a/input.txt");
    let result = plot_course(input);
    let product = result.x * result.y;
    format!("x: {}, y: {}, product: {}", result.x, result.y, product)
}

#[derive(PartialEq, Eq, Debug)]
struct Coords {
    x: i32,
    y: i32,
}

struct Command {
    direction: String,
    amount: i32,
}

impl Command {
    fn new<'a>(mut parts: impl Iterator<Item=&'a str>) -> Command {
        let direction = parts.next().expect("Need to provide direction!").to_string();
        let amount = parts.next().expect("Need to provide amount!")
            .parse::<i32>().expect("The amount must be an integer number!");
        if parts.next().is_some() { panic!("A line should contain of two items!") }
        Command { direction, amount }
    }
}

fn plot_course<S: AsRef<str>>(input: impl Iterator<Item=S>) -> Coords {
    let commands = input.map(|line| Command::new(line.as_ref().split_whitespace()));
    let mut coords = Coords { x: 0, y: 0 };

    for command in commands {
        match command.direction.as_str() {
            "forward" => coords.x += command.amount,
            "down" => coords.y += command.amount,
            "up" => coords.y -= command.amount,
            s => panic!("Wrong direction: {}", s)
        }
    }

    coords
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
        let result = plot_course(INPUT.lines());
        assert_that!(result).is_equal_to(Coords { x: 15, y: 10 });
    }
}