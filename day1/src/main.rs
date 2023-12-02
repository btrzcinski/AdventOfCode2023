use std::{fs::File, io::Read};

use data_loader::pick_data_file;
use regex::Regex;

fn read_input_file() -> String {
    let file_path = pick_data_file();
    let display = file_path.display();
    let mut file = match File::open(file_path.as_path()) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read {}: {}", display, why),
        Ok(_) => (),
    }
    return s;
}

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

fn main() {
    env_logger::init();

    let value_sum: u32 = read_input_file()
        .lines()
        .filter(|l| !l.is_empty())
        .map(calibration_value)
        .sum();
    println!("Sum: {value_sum}");
}
