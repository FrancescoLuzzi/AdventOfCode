use aoc_utils;
// https://adventofcode.com/2023/day/1

struct Mapping<'a>(&'a str, char);

static MAPPINGS: [Mapping; 9] = [
    Mapping("one", '1'),
    Mapping("two", '2'),
    Mapping("three", '3'),
    Mapping("four", '4'),
    Mapping("five", '5'),
    Mapping("six", '6'),
    Mapping("seven", '7'),
    Mapping("eight", '8'),
    Mapping("nine", '9'),
];

fn parse_calibration(line: &str) -> String {
    let line_legth = line.len();
    (0..line_legth)
        .flat_map(|start| {
            let curr_line = &line[start..line_legth];
            let first_char = curr_line.chars().next()?;
            for mapping in &MAPPINGS {
                if curr_line.starts_with(mapping.0) || first_char == mapping.1 {
                    return Some(mapping.1);
                }
            }
            None
        })
        .collect::<String>()
}

fn read_calibration(line: &str) -> Option<u32> {
    let mut found_calibrations = line.chars().flat_map(|c| c.to_digit(10));
    let first = found_calibrations.next()?;
    let second = found_calibrations.last().unwrap_or(first);
    Some(first * 10 + second)
}

fn main() {
    let iter_lines = aoc_utils::load_input_file("input.txt");
    let result: u32 = iter_lines
        .map(|line| parse_calibration(&line))
        .flat_map(|line| read_calibration(&line))
        .sum();
    print!("{result}");
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::{parse_calibration, read_calibration};

    #[rstest]
    #[case("46threevqs8114", Some(44))]
    #[case("6lsjdagkjsdfog", Some(66))]
    #[case("dshjgdfjgksdjf", None)]
    fn test_read_calibration(#[case] input: &str, #[case] expected: Option<u32>) {
        assert_eq!(read_calibration(input), expected);
    }

    #[rstest]
    #[case("threevqs8114", "38114")]
    #[case("eightwone", "821")]
    #[case("6lsjdagkjsdfog", "6")]
    #[case("dshjgdfjgksdjf", "")]
    fn test_parse_calibration(#[case] input: &str, #[case] expected: String) {
        assert_eq!(parse_calibration(input), expected);
    }
}
