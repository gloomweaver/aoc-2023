fn process(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            line.replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
        })
        .map(|s| {
            let first = s.chars().next().unwrap();
            let last = s.chars().last().unwrap();
            format!("{}{}", first, last).parse::<i64>().unwrap()
        })
        .sum()
}

fn main() {
    let input = r#"two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen"#;
    let res = process(input);
    println!("Result: {}", res);
}
