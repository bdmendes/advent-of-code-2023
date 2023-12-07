#[derive(Debug, Clone, Copy)]
struct Race {
    time: u64,
    record_distance: u64,
}

#[derive(Debug, Clone, Copy)]
struct RaceStrategy {
    hold_speed: u64,
}

fn parse_races(lines: &[&str]) -> Vec<Race> {
    let extract_numbers = |line: &str| {
        line.split(":")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    };

    let times = extract_numbers(lines.into_iter().nth(0).unwrap());

    let distances = extract_numbers(lines.into_iter().nth(1).unwrap());

    (0..times.len())
        .into_iter()
        .map(|i| Race {
            time: times[i],
            record_distance: distances[i],
        })
        .collect::<Vec<_>>()
}

fn valid_race_strategies(race: Race) -> Vec<RaceStrategy> {
    let mut strategies = vec![];
    for holding_time in 1..(race.time) {
        let race_distance = (race.time - holding_time) * holding_time;
        if race_distance > race.record_distance {
            strategies.push(RaceStrategy {
                hold_speed: holding_time,
            })
        }
    }
    strategies
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let lines = file.lines().collect::<Vec<&str>>();

    let races = parse_races(&lines);

    let winning_strategies_product = races
        .iter()
        .map(|r| valid_race_strategies(*r).len())
        .product::<usize>();

    println!("Winning strategies product: {}", winning_strategies_product);

    let lines_without_spaces = lines
        .iter()
        .map(|l| l.replace(" ", ""))
        .into_iter()
        .collect::<Vec<_>>();
    let lines_without_spaces = lines_without_spaces
        .iter()
        .map(|l| l.as_str())
        .collect::<Vec<&str>>();

    let single_race = parse_races(&lines_without_spaces)
        .into_iter()
        .nth(0)
        .unwrap();

    let winning_strategy = valid_race_strategies(single_race).len();

    println!("Kerning winning strategy: {}", winning_strategy);
}
