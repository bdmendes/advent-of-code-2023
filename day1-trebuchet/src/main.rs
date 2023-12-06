fn calibration_value_digits(line: &str) -> u32 {
    let digits = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();
    let first_digit = digits[0];
    let last_digit = digits[digits.len() - 1];
    first_digit * 10 + last_digit
}

fn calibration_value_str(line: &str) -> u32 {
    let numbers = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let str_digits = numbers
        .iter()
        .flat_map(|n| line.match_indices(n))
        .map(|(i, n)| (i, numbers.iter().position(|&x| x == n).unwrap() + 1))
        .collect::<Vec<_>>();

    let number_digits = line
        .chars()
        .enumerate()
        .filter_map(|(i, c)| {
            c.is_digit(10)
                .then(|| (i, c.to_digit(10).unwrap() as usize))
        })
        .collect::<Vec<_>>();

    let mut all_digits = str_digits
        .iter()
        .chain(number_digits.iter())
        .collect::<Vec<_>>();

    all_digits.sort_by_key(|(i, _)| *i);

    println!("{:?}", all_digits);

    let first_digit = all_digits[0].1;
    let last_digit = all_digits[all_digits.len() - 1].1;
    first_digit as u32 * 10 + last_digit as u32
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let lines = file.lines();

    let calibration_sum_digits = lines.clone().map(calibration_value_digits).sum::<u32>();
    println!("Calibration sum: {}", calibration_sum_digits);

    let calibration_sum_str = lines.clone().map(calibration_value_str).sum::<u32>();
    println!("Calibration sum: {}", calibration_sum_str);
}
