use std::cmp::min;

fn numbers(line: &str) -> (Vec<u64>, Vec<u64>) {
    let numbers_section = line.split(':').nth(1).unwrap();
    let winning_numbers = numbers_section
        .split('|')
        .nth(0)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let losing_numbers = numbers_section
        .split('|')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    (winning_numbers, losing_numbers)
}

fn prize(winning: &[u64], cards: &[u64]) -> u64 {
    let intersection = winning
        .iter()
        .filter(|x| cards.contains(x))
        .collect::<Vec<&u64>>();
    if intersection.len() == 0 {
        return 0;
    }
    2u64.pow(intersection.len() as u32 - 1)
}

fn game(prizes: &[u64]) -> Vec<u64> {
    let mut rewards = vec![1; prizes.len()];

    for i in 0..prizes.len() {
        let prize = if prizes[i] == 0 {
            0
        } else {
            prizes[i].ilog2() as u64 + 1
        };
        for j in (i + 1)..(min(i + 1 + prize as usize, rewards.len())) {
            rewards[j] += rewards[i];
        }
    }

    rewards
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let lines = file.lines().collect::<Vec<&str>>();

    let all_numbers = lines
        .iter()
        .map(|line| numbers(line))
        .collect::<Vec<(Vec<u64>, Vec<u64>)>>();

    let prizes = all_numbers.iter().map(|n| prize(&n.0, &n.1));
    let prizes_sum = prizes.clone().sum::<u64>();

    println!("Prizes sum: {}", prizes_sum);

    let game = game(prizes.collect::<Vec<u64>>().as_slice());
    println!("Rewards: {:?} (sum={})", game, game.iter().sum::<u64>());
}
