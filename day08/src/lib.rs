#![allow(dead_code, unused_imports)]
mod tests;
use std::collections::HashMap;
use num::integer::lcm;

struct Mapping<'a> {
    pub dir: [&'a str; 2],
}

impl<'a> Mapping<'a> {
    fn from((left, right): (&'a str, &'a str)) -> Self {
        Self{dir: [left, right]}
    }
}

fn get_map(line: &str) -> (&str, (&str, &str)) {
    let mut a = line.split(" = ");
    let key = a.next().unwrap();
    a = a.next().unwrap().split(", ");
    let left = a.next().unwrap().split("(").nth(1).unwrap();
    let right = a.next().unwrap().split(")").nth(0).unwrap();

    (key, (left, right))
}

pub fn part1(prompt: String) -> u64 {
    let mut keymap: HashMap<&str, Mapping> = HashMap::new();

    let instructions: Vec<u8> = prompt
        .lines().nth(0).unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
             _ => panic!()
        })
        .collect();
    let inst_n = instructions.len();
    
    let maps = prompt
        .lines()
        .skip(2)
        .map(|line| get_map(line));

    for map in maps {
        keymap.insert(map.0, Mapping::from(map.1));
    }

    let end = "ZZZ";
    let mut current = "AAA";
    let mut i = 0;
    
    while current != end {
        current = keymap.get(current).unwrap().dir[instructions[i%inst_n] as usize];
        i += 1;
    }

    i as u64
}

pub fn is_end(key: &str) -> bool {
    key.ends_with('Z')
}

pub fn part2(prompt: String) -> u64 {
    let mut keymap: HashMap<&str, Mapping> = HashMap::new();

    let instructions: Vec<u8> = prompt
        .lines().nth(0).unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
             _ => panic!()
        })
        .collect();
    let inst_n = instructions.len();
    
    let maps = prompt
        .lines()
        .skip(2)
        .map(|line| get_map(line));

    for map in maps.clone() {
        keymap.insert(map.0, Mapping::from(map.1));
    }

    maps
        .map(|(key, _)| key)
        .filter(|key| key.ends_with('A'))
        .map(|mut key| {
            let mut i = 0;
            while !is_end(key) {
                key = keymap.get(key).unwrap().dir[instructions[i%inst_n] as usize];
                i += 1;
            }
            i
        })
        .fold(1, |acc, x| lcm(acc, x as u64))
}
