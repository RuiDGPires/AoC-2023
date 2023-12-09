/*
 * I was trying to refactor the code to range operations instead of bruteforcing because it was
 * taking too long.
 * Then I reallized I should try to run it in release mode.
 * It did it in a couple minutes...
 * I'm leaving this file as is, without cleaning
 */

#![allow(dead_code, unused_imports)]
mod tests;
use std::collections::HashSet;
use std::hash::{Hash};

#[derive(Clone, Copy, Debug)]
struct Mapping {
    pub destination: u64,
    pub source: u64,
    pub range: u64,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Range (u64, u64);

impl Range {
    fn unfold(&self) -> std::ops::Range<u64>{
        self.0 .. self.1
    } 
}

trait SeedMap {
    fn seed(&self, seed: u64) -> Option<u64>;
}

impl Mapping {
    fn new(destination: u64, source: u64, range: u64) -> Self {
        Self {source: source, destination: destination, range: range}
    }

    fn from(line: &str) -> Option<Self> {
        let vals = line.split_whitespace();

        let mut vals = vals
            .map(|x| x.parse::<u64>().ok())
            .filter(|x| x.is_some())
            .map(|x| x.unwrap());

        if vals.clone().count() != 3 {
            return None;
        }

        Some(Self::new(vals.nth(0).unwrap(), vals.nth(0).unwrap(), vals.nth(0).unwrap()))
    }
}

impl SeedMap for Mapping {
    fn seed(&self, seed: u64) -> Option<u64> {
        if (self.source..(self.source + self.range)).contains(&seed) {
            return Some(seed - self.source + self.destination);
        }
        None
    }
}

impl SeedMap for &Mapping {
    fn seed(&self, seed: u64) -> Option<u64> {
        if (self.source..(self.source + self.range)).contains(&seed) {
            return Some(seed - self.source + self.destination);
        }
        None
    }
}

fn get_map<T: Clone>(mappings: T, seed: u64) -> u64 
    where T: IntoIterator, T::Item: SeedMap {
    for map in mappings.clone() {
        if let Some(res) = map.seed(seed) {
            return res;
        }
    }
    seed
}

pub fn part1(prompt: String) -> u64 {
    let seeds = prompt.lines().nth(0).unwrap().split(":").nth(1).unwrap().split_whitespace().map(|x| x.parse::<u64>().unwrap());

    let mut min = u64::MAX;
    for seed in seeds {
        let mut mapped = seed;
        for dest in prompt.split(":").skip(2) {
            let mapping = dest
                .lines()
                .map(|line| Mapping::from(line))
                .filter(|x| x.is_some())
                .map(|x| x.unwrap());
            mapped = get_map(mapping, mapped);
        }

        if mapped < min {
            min = mapped
        }
    }
    
    min
}

struct RangeMap {
    range: Range,
    offset: i64
}

impl RangeMap {
    fn from(mapping: Mapping) -> Self {
        Self{range: Range(mapping.source, mapping.source + mapping.range), offset: mapping.destination as i64 - mapping.source as i64} 
    }
}

impl SeedMap for RangeMap {
    fn seed(&self, seed: u64) -> Option<u64> {
        if (self.range).unfold().contains(&seed) {
            return Some((seed as i64 + self.offset) as u64);
        }
        None
    }
}


impl SeedMap for &RangeMap {
    fn seed(&self, seed: u64) -> Option<u64> {
        if (self.range).unfold().contains(&seed) {
            return Some((seed as i64 + self.offset) as u64);
        }
        None
    }
}

pub fn part2(prompt: String) -> u64 {
    let mut seed_ranges = prompt.lines().nth(0).unwrap().split(":").nth(1).unwrap().split_whitespace().map(|x| x.parse::<u64>().unwrap());
    
    let mut ranges: HashSet<Range> = HashSet::new();

    while let Some(base) = seed_ranges.next() {
        ranges.insert(Range(base, base + seed_ranges.next().unwrap()));
    }

    let maps = prompt.split(":").skip(2)
        .map(|layer|
                layer
                    .lines()
                    .map(|line| Mapping::from(line))
                    .filter(|x| x.is_some())
                    .map(|x| RangeMap::from(x.unwrap()))
                    .collect()
            )
        .collect::<Vec<Vec<RangeMap>>>();
    
    let mut min = u64::MAX;
    for range in ranges {
        for seed in range.0 .. range.1 {
            let mut mapped = seed;
            for mapping in &maps {
                mapped = get_map(mapping.iter(), mapped);
            }
            if mapped < min {
                min = mapped
            }
        }
    }

    min
}
