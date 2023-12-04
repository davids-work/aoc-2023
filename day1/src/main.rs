fn main() {
    let input = include_str!("input.txt");
    let sum: u32 = input
        .lines()
        .map(|l| l.chars().filter(|c| c.is_digit(10)))
        .map(|mut ld| {
            let first = ld.next().unwrap();
            let last = ld.last().unwrap_or(first);
            let number = format!("{}{}", first, last);
            number.parse::<u32>().unwrap()
        })
        .sum();

    println!("Answer part 1: {}", sum);
}
