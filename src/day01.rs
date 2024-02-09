use std::fs;

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

fn convert_str_nums_to_nums(s: String) -> String {
    return s
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9");
}

fn get_input_data() -> Vec<String> {
    let contents = fs::read_to_string("/home/nixos/vcs/eksno/advent-of-code-rust/src/day01/input.txt")
        .expect("Should have been able to read the file");

    return contents.split('\n').map(String::from).collect();
}

pub fn solve() {
    let calibration_values: Vec<String> = get_input_data();

    let mut total = 0;
    for s_raw in &calibration_values {
        let s = convert_str_nums_to_nums((&s_raw).to_string());
        let first = get_first_digit(&s);
        let last = get_last_digit(&s);
        let val = (first * 10) + last;
        println!("{} - {} - {}", val, s, s_raw);
        total = total + val;
    }

    println!("Result: {}", total);
}
