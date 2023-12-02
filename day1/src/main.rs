use std::{env, fs::File, io::Read, path::Path};

fn read_input_file() -> String {
    let file_name = env::args()
        .skip(1)
        .next()
        .expect("Input file name required");
    let path = Path::new(&file_name);
    let display = path.display();
    let mut file = match File::open(path) {
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

fn calibration_value(line: &str) -> u32 {
    let mut digits = line.chars().filter(|c| c.is_numeric());
    let first = match digits.next() {
        Some(c) => c,
        None => panic!("No digit in line: {}", line),
    };
    let last = digits.next_back().unwrap_or(first);
    u32::from_str_radix(&format!("{}{}", first, last), 10)
        .expect("Can't parse number from two digits in line")
}

fn main() {
    let value_sum: u32 = read_input_file()
        .lines()
        .filter(|l| !l.is_empty())
        .map(calibration_value)
        .sum();
    println!("Sum: {value_sum}");
}
