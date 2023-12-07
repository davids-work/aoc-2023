use aho_corasick::AhoCorasick;

fn calculate_sum(input: &str, extractor: fn(&str) -> u32) -> u32 {
    input.lines().map(extractor).sum()
}

fn day1(line: &str) -> u32 {
    let mut ld = line.chars().filter(char::is_ascii_digit);
    let first = ld.next().unwrap();
    let last = ld.last().unwrap_or(first);
    let number = format!("{}{}", first, last);
    number.parse::<u32>().unwrap()
}

fn to_digit(digit_str: &str) -> Option<&str> {
    match digit_str {
        "one" => Some("1"),
        "two" => Some("2"),
        "three" => Some("3"),
        "four" => Some("4"),
        "five" => Some("5"),
        "six" => Some("6"),
        "seven" => Some("7"),
        "eight" => Some("8"),
        "nine" => Some("9"),
        _ => None,
    }
}

fn day2(line: &str) -> u32 {
    let patterns = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];

    let ac = AhoCorasick::new(patterns).unwrap();
    let mut iter = ac.find_overlapping_iter(line);
    let first_match = iter.next().unwrap().pattern().as_usize();
    let last_match = iter
        .last()
        .map(|m| m.pattern().as_usize())
        .unwrap_or(first_match);
    let first_digit = to_digit(patterns[first_match]).unwrap_or(patterns[first_match]);
    let second_digit = to_digit(patterns[last_match]).unwrap_or(patterns[last_match]);
    let number_str = format!("{}{}", first_digit, second_digit);
    number_str.parse().unwrap()
}

fn main() {
    let input = include_str!("input.txt");
    let sum_part1 = calculate_sum(input, day1);

    println!("Answer part 1: {}", sum_part1);

    let sum_part2 = calculate_sum(input, day2);
    println!("Answer part 2: {}", sum_part2);
}
