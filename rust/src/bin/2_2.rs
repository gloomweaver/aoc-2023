fn process(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;
            let parts = line.split(":");

            let cubes = parts.last().unwrap().trim();
            cubes.split(";").for_each(|cube| {
                cube.split(", ").for_each(|c| {
                    let parts = c.trim().split(" ").collect::<Vec<_>>();
                    match parts[..] {
                        [count, "red"] => {
                            if count.parse::<i64>().unwrap() >= max_red {
                                max_red = count.parse::<i64>().unwrap();
                            }
                        }
                        [count, "green"] => {
                            if count.parse::<i64>().unwrap() >= max_green {
                                max_green = count.parse::<i64>().unwrap();
                            }
                        }
                        [count, "blue"] => {
                            if count.parse::<i64>().unwrap() >= max_blue {
                                max_blue = count.parse::<i64>().unwrap();
                            }
                        }
                        _ => (),
                    }
                });
            });
            max_red * max_green * max_blue
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
    println!("Result: {:?}", res);
}
