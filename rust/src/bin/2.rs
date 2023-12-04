fn process(input: &str) -> i64 {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    input
        .lines()
        .map(|line| {
            let mut parts = line.split(":");
            let id = parts
                .next()
                .and_then(|s| s.trim().split(" ").nth(1))
                .and_then(|s| s.parse::<i64>().ok())
                .unwrap();
            let cubes = parts.next().unwrap().trim();
            let valid = cubes.split(";").all(|cube| {
                cube.split(", ").all(|c| {
                    let parts = c.trim().split(" ").collect::<Vec<_>>();
                    match parts[..] {
                        [count, "red"] => count.parse::<i64>().unwrap() <= max_red,
                        [count, "green"] => count.parse::<i64>().unwrap() <= max_green,
                        [count, "blue"] => count.parse::<i64>().unwrap() <= max_blue,
                        _ => false,
                    }
                })
            });
            if valid {
                id
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    let res = process(input);
    println!("Result: {}", res);
}
