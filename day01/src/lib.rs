#![allow(dead_code, unused_imports)]
mod tests;

fn part1(prompt: String) -> u64 {
    let mut total = 0;
    for line in prompt.lines() {
        let mut n = "".to_string();
        for c in line.chars() {
            if c.is_digit(10) {
                n.push(c);
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_digit(10) {
                n.push(c);
                break;
            }
        }
        
        total += n.parse::<u64>().unwrap();
    }

    total
}

fn part2(input: String) -> u64 {
    let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let digits_trans = ["111", "222", "33333", "4444", "5555", "666", "77777", "88888", "9999"];

    let mut total = 0;
    for line in input.lines() {
        let mut n = "".to_string();
        let mut pos = line.len();
        let mut current = ' ';

        // --- first
        for (digit, t) in digits.iter().zip(digits_trans) {
            let new_line = line.replace(digit, t);

            for (i, c) in new_line.chars().enumerate() {
                if c.is_digit(10) && i < pos {
                     current = c;
                     pos = i;
                }
            }
        }
        n.push(current);

        // --- last
        
        let mut pos = line.len();
        let mut current = ' ';

        for (digit, t) in digits.iter().zip(digits_trans) {
            let new_line = line.replace(digit, t);

            for (i, c) in new_line.chars().rev().enumerate() {
                if c.is_digit(10) && i < pos {
                     current = c;
                     pos = i;
                }
            }
        }

        n.push(current);

        total += n.parse::<u64>().unwrap();
    }

    total
}
