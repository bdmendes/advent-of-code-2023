use std::collections::HashMap;

type Id = u32;

fn parse_game(line: &str) -> (Id, Vec<Vec<(u32, String)>>) {
    let id = line
        .split(':')
        .nth(0)
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse::<Id>()
        .unwrap();
    let grabs = line
        .split(':')
        .nth(1)
        .unwrap()
        .split(';')
        .collect::<Vec<_>>();

    let mut grabs_vec = vec![];

    for grab in grabs {
        let mut grab_vec = vec![];
        let reveals = grab
            .split(",")
            .map(|r| r.split_whitespace().collect::<Vec<_>>());
        for reveal in reveals {
            let number = reveal[0].parse::<u32>().unwrap();
            let color = reveal[1];
            grab_vec.push((number, color.to_string()));
        }
        grabs_vec.push(grab_vec);
    }

    (id, grabs_vec)
}

fn validate_game(line: &str) -> Option<u32> {
    let (id, grabs) = parse_game(line);

    for grab in grabs {
        for reveal in grab {
            let number = reveal.0;
            let color = reveal.1;
            if (number > 12 && color == "red")
                || (number > 13 && color == "green")
                || (number > 14 && color == "blue")
            {
                return None;
            }
        }
    }

    Some(id)
}

fn minimum_game_power(line: &str) -> u32 {
    let (_, grabs) = parse_game(line);
    let mut map = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
    for grab in grabs {
        for cube in grab {
            let (count, color) = cube;
            let value = map.get_mut(color.as_str()).unwrap();
            if *value < count {
                *value = count;
            }
        }
    }
    map.iter().map(|(_, count)| count).product()
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let lines = file.lines().collect::<Vec<_>>();

    let valid_games_sum = lines.iter().filter_map(|s| validate_game(s)).sum::<u32>();
    println!("Valid games sum: {}", valid_games_sum);

    let game_powers_sum = lines.iter().map(|s| minimum_game_power(s)).sum::<u32>();
    println!("Powers sum: {}", game_powers_sum);
}
