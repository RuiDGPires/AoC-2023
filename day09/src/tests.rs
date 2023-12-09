use crate::*;
use std::path::PathBuf;
use std::fs;

#[test]
fn part1_prompt() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("inputs/prompt.txt");

    let prompt = fs::read_to_string(path).unwrap();

    assert_eq!(part1(prompt.to_string()), 1853145119);
}

#[test]
fn part2_prompt() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("inputs/prompt.txt");

    let prompt = fs::read_to_string(path).unwrap();

    assert_eq!(part2(prompt.to_string()), 923);
}

#[test]
fn part1_input1() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("inputs/input1.txt");

    let prompt = fs::read_to_string(path).unwrap();

    assert_eq!(part1(prompt.to_string()), 114);
}

#[test]
fn part2_input1() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("inputs/input1.txt");

    let prompt = fs::read_to_string(path).unwrap();

    assert_eq!(part2(prompt.to_string()), 2);
}
