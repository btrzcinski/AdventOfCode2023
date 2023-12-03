use std::path::Path;

use advent_tools::*;
use regex::Regex;

fn extract_digit_from_prefix(line: &str) -> Option<char> {
    let re = Regex::new(r"^(?<digit>one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    re.captures(line)
        .map(|c| match c.name("digit").map(|m| m.as_str()) {
            Some("one") => '1',
            Some("two") => '2',
            Some("three") => '3',
            Some("four") => '4',
            Some("five") => '5',
            Some("six") => '6',
            Some("seven") => '7',
            Some("eight") => '8',
            Some("nine") => '9',
            Some(digit) => digit
                .chars()
                .next()
                .expect("single character digit in match"),
            None => panic!("Unexpected match without digit group"),
        })
}

fn extract_digits_from_line(line: &str) -> Vec<char> {
    line.char_indices()
        .map(|(i, _)| &line[i..])
        .filter_map(extract_digit_from_prefix)
        .collect()
}

fn calibration_value(line: &str) -> u32 {
    let digits = extract_digits_from_line(line);
    log::debug!("Mapped {line} to {digits:#?}");
    let first = match digits.first() {
        Some(c) => c,
        None => panic!("No digit in line: {}", line),
    };
    let last = digits.last().unwrap_or(first);
    u32::from_str_radix(&format!("{}{}", first, last), 10)
        .expect("Can't parse number from two digits in line")
}

fn get_calibration_value_sum(path: &Path) -> u32 {
    read_input_file(path)
    .lines()
    .filter(|l| !l.is_empty())
    .map(calibration_value)
    .sum()
}

fn main() {
    env_logger::init();

    let file_path = pick_data_file();
    let value_sum: u32 = report_runtime(|| get_calibration_value_sum(file_path.as_path()));
    println!("Sum: {value_sum}");
}
