#![allow(dead_code, unused_imports)]
mod tests;

pub fn part1(prompt: String) -> u64 {
    let races: Vec<(u64, u64)> = prompt
        .lines()
        .nth(0).unwrap()
        .split_whitespace()
        .zip(
            prompt.lines()
            .nth(1).unwrap()
            .split_whitespace()
            )
        .skip(1)
        .map(|(dur, rec)| (dur.parse().unwrap(), rec.parse().unwrap()))
        .collect()
        ;

    let mut total = 1;
    for (duration, record) in races {
        let end       = f64::sqrt((duration * duration) as f64 / 4.- record as f64) + duration as f64 / 2.;
        let begin     = -f64::sqrt((duration * duration) as f64 / 4.- record as f64) + duration as f64 / 2.;
        
        total *= end.ceil() as u64 - begin.floor() as u64 - 1; 
    }
    
    total
}

pub fn part2(prompt: String) -> u64 {
    let (duration, record) = prompt
        .lines()
        .nth(0).unwrap()
        .split_whitespace()
        .zip(
            prompt.lines()
            .nth(1).unwrap()
            .split_whitespace()
            )
        .skip(1)
        .fold(("".to_owned(), "".to_owned()), |(a_dur, a_rec), (dur, rec)| (format!("{}{}", a_dur, dur), format!("{}{}", a_rec, rec)))
        ;

    let (duration, record): (u64, u64) = (duration.parse().unwrap(), record.parse().unwrap());
    let end       = f64::sqrt((duration * duration) as f64 / 4.- record as f64) + duration as f64 / 2.;
    let begin     = -f64::sqrt((duration * duration) as f64 / 4.- record as f64) + duration as f64 / 2.;
    
    end.ceil() as u64 - begin.floor() as u64 - 1
}
