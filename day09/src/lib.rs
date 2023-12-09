#![allow(dead_code, unused_imports)]
#[cfg(test)]
mod tests;
use regex::Regex;

fn solve_line<T>(line: T)  -> i64 where T: Iterator<Item=i64>{
    let mut line = line.collect::<Vec<i64>>();
    let mut values = Vec::<i64>::new();
    let len = line.len();
    let mut j = 0;
    
    loop {
        if {
            let mut res = true;
            for i in j .. len {
                res = res && line[i] == 0;
            }
            res
        } {
            break;
        }
        values.push(line[len - 1]);
        let mut i = len - 1;
        while i > j {
            line[i] = line[i] - line [i-1];
            i -= 1;
        }
        j += 1;
    }
    
    values.iter().sum()
}

pub fn part1(prompt: String) -> i64 {
    prompt
        .lines()
        .map(|line| line.split_whitespace().map(|n| n.parse::<i64>().unwrap()))
        .map(|line| solve_line(line))
        .sum()
}

pub fn part2(prompt: String) -> i64 {
    prompt
        .lines()
        .map(|line| line.split_whitespace().map(|n| n.parse::<i64>().unwrap()))
        .map(|line| solve_line(line.rev()))
        .sum()
}
