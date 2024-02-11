use std::fs;
use regex::Regex;

fn get_first_digit(s: &str) -> u32 {
    for c in s.chars() {
        if c.is_digit(10) {
            match c.to_digit(10) {
                Some(digit) => return digit,
                None => println!("not a digit"),
            }
        }
    }
    return 0;
}

fn get_last_digit(s: &str) -> u32 {
    for c in s.chars().rev() {
        if c.is_digit(10) {
            match c.to_digit(10) {
                Some(digit) => return digit,
                None => println!("not a digit"),
            }
        }
    }
    return 0;
}

fn word_to_number(word: &str) -> String {
    match word.to_lowercase().as_str() {
        "zero" => "0".to_string(),
        "one" => "1".to_string(),
        "two" => "2".to_string(),
        "three" => "3".to_string(),
        "four" => "4".to_string(),
        "five" => "5".to_string(),
        "six" => "6".to_string(),
        "seven" => "7".to_string(),
        "eight" => "8".to_string(),
        "nine" => "9".to_string(),
        _ => word.to_string(),
    }
}

fn convert_last_str_num_to_num(s: String) -> String {
    let re = Regex::new(r"(?i)(zero|one|two|three|four|five|six|seven|eight|nine)([^a-zA-Z0-9]|$)").unwrap();

    return re.replace_all(&s, |captures: &regex::Captures| {
        let matched_text = &captures[0];
        word_to_number(matched_text)
    }).to_string();
}

fn convert_first_str_num_to_num(s: String) -> String {
    let re = Regex::new(r"(?i)(zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    return re.replace_all(&s, |captures: &regex::Captures| {
        let matched_text = &captures[0];
        word_to_number(matched_text)
    }).to_string();
}

fn get_input_data() -> Vec<String> {
    let contents = fs::read_to_string("/home/eksno/vcs/eksno/aoc-2023-rust/src/day01/input.txt")
        .expect("Should have been able to read the file");

    return contents.split('\n').map(String::from).collect();
}

pub fn solve() {
    let calibration_values: Vec<String> = get_input_data();

    let mut total = 0;
    for s_raw in &calibration_values {
        let first = get_first_digit(&convert_first_str_num_to_num((&s_raw).to_string()));
        let last = get_last_digit(&convert_last_str_num_to_num((&s_raw).to_string()));
        let val = (first * 10) + last;
        println!("{} - {} {} - {}", val, first, last, s_raw);
        total = total + val;
    }

    println!("Result: {}", total);
}
