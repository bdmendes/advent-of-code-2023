use std::char;

use regex::Regex;

struct PartNumber {
    number: u32,
    start_idx: usize,
    end_idx: usize,
    line_idx: usize,
}

#[derive(Debug)]
struct CharPosition {
    line_idx: usize,
    char_idx: usize,
}

fn is_part_number(lines: Vec<&str>, line_idx: usize, start_idx: usize, end_idx: usize) -> bool {
    fn is_symbol(c: char) -> bool {
        !c.is_digit(10) && c != '.'
    }

    for i in start_idx..end_idx {
        // Top left
        if i == start_idx
            && line_idx > 0
            && i > 0
            && is_symbol(lines[line_idx - 1].chars().nth(i - 1).unwrap())
        {
            return true;
        }

        // Bottom left
        if i == start_idx
            && line_idx < lines.len() - 1
            && i > 0
            && is_symbol(lines[line_idx + 1].chars().nth(i - 1).unwrap())
        {
            return true;
        }

        // Top
        if line_idx > 0 && is_symbol(lines[line_idx - 1].chars().nth(i).unwrap()) {
            return true;
        }

        // Bottom
        if line_idx < lines.len() - 1 && is_symbol(lines[line_idx + 1].chars().nth(i).unwrap()) {
            return true;
        }

        // Left
        if i == start_idx && i > 0 && is_symbol(lines[line_idx].chars().nth(i - 1).unwrap()) {
            return true;
        }

        // Right
        if i == end_idx - 1
            && i < lines[line_idx].len() - 1
            && is_symbol(lines[line_idx].chars().nth(i + 1).unwrap())
        {
            return true;
        }

        // Top right
        if i == end_idx - 1
            && line_idx > 0
            && i < lines[line_idx - 1].len() - 1
            && is_symbol(lines[line_idx - 1].chars().nth(i + 1).unwrap())
        {
            return true;
        }

        // Bottom right
        if i == end_idx - 1
            && line_idx < lines.len() - 1
            && i < lines[line_idx + 1].len() - 1
            && is_symbol(lines[line_idx + 1].chars().nth(i + 1).unwrap())
        {
            return true;
        }
    }

    false
}

fn part_numbers(lines: Vec<&str>) -> Vec<PartNumber> {
    let numbers_regex = Regex::new(r"\d+").unwrap();
    let mut numbers = vec![];
    for (line_idx, line) in lines.iter().enumerate() {
        let candidates = numbers_regex.find_iter(line);
        for candidate in candidates {
            if is_part_number(lines.clone(), line_idx, candidate.start(), candidate.end()) {
                let number = candidate.as_str().parse::<u32>().unwrap();
                numbers.push(PartNumber {
                    number,
                    start_idx: candidate.start(),
                    end_idx: candidate.end(),
                    line_idx,
                });
            }
        }
    }
    numbers
}

fn gear_ratios_candidates(lines: Vec<&str>) -> Vec<CharPosition> {
    let mut ratios = vec![];
    for (line_idx, line) in lines.into_iter().enumerate() {
        for (c_idx, c) in line.chars().enumerate() {
            if c == '*' {
                ratios.push(CharPosition {
                    line_idx,
                    char_idx: c_idx,
                });
            }
        }
    }
    ratios
}

fn gear_ratios(candidates: Vec<CharPosition>, part_numbers: Vec<PartNumber>) -> Vec<u32> {
    let mut ratios = vec![];
    for candidate in candidates {
        let adjacent_numbers = part_numbers
            .iter()
            .filter(|p| {
                ((p.line_idx.saturating_sub(1))..(p.line_idx.saturating_add(2)))
                    .contains(&candidate.line_idx)
                    && ((p.start_idx.saturating_sub(1))..(p.end_idx.saturating_add(1)))
                        .contains(&candidate.char_idx)
            })
            .collect::<Vec<&PartNumber>>();
        if adjacent_numbers.len() == 2 {
            ratios.push(adjacent_numbers[0].number * adjacent_numbers[1].number);
        }
    }
    ratios
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let lines = file.lines().collect::<Vec<&str>>();

    let part_numbers = part_numbers(lines.clone());
    println!(
        "Part numbers sum: {:?}",
        part_numbers.iter().map(|p| p.number).sum::<u32>()
    );

    let gear_candidates = gear_ratios_candidates(lines);
    println!("Gear candidates: {:?}", gear_candidates);

    let gear_ratios = gear_ratios(gear_candidates, part_numbers);
    println!("Gear ratios: {:?}", gear_ratios);

    println!("Gear ratios sum: {:?}", gear_ratios.iter().sum::<u32>());
}
