#![allow(dead_code, unused_imports)]
mod tests;
use regex::Regex;

fn check(max: u32, line: &str, re: &Regex) -> bool {
    if let Some(value) = re.captures(line).and_then(|cap| {
        cap.name("n").map(|n| n.as_str().parse::<u32>().unwrap())
    }) {
        if max >= value {
            return true;
        } else {
            return false;
        }
    }
    true
}

fn get(line: &str, re: &Regex) -> u32 {
    if let Some(value) = re.captures(line).and_then(|cap| {
        cap.name("n").map(|n| n.as_str().parse::<u32>().unwrap())
    }) {
        value
    } else {
        0
    }
}

fn get_id(line: &str) -> u32 {
    if let Some(value) = Regex::new(r"Game (?<n>\d+)").unwrap().captures(line).and_then(|cap| {
        cap.name("n").map(|n| n.as_str().parse::<u32>().unwrap())
    }) {
        return value;
    }

    0
}

fn part1(prompt: String) -> u32 {
    let re_red = Regex::new(r"(?<n>\d+) red").unwrap();
    let re_green = Regex::new(r"(?<n>\d+) green").unwrap();
    let re_blue = Regex::new(r"(?<n>\d+) blue").unwrap();

    let mut total = 0;

    for line in prompt.lines() {
        let mut valid = true;
        for set in line.split(";") {
            if !check(12, set, &re_red) {valid = false;}
            if !check(13, set, &re_green) {valid = false;}
            if !check(14, set, &re_blue) {valid = false;}
        }

        if valid {
            total += get_id(line);
        }
    }

    total
}

fn part2(prompt: String) -> u32 {
    let re_red = Regex::new(r"(?<n>\d+) red").unwrap();
    let re_green = Regex::new(r"(?<n>\d+) green").unwrap();
    let re_blue = Regex::new(r"(?<n>\d+) blue").unwrap();

    let mut total = 0;

    for line in prompt.lines() {
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green  = 0;

        for set in line.split(";") {
            let m = get(set, &re_red);
            if m > max_red {
                max_red = m;
            }
            let m = get(set, &re_green);
            if m > max_green {
                max_green = m;
            }

            let m = get(set, &re_blue);
            if m > max_blue {
                max_blue = m;
            }
        }
        total += max_red * max_blue * max_green
    }
    total
}
