use crate::*;
use std::path::PathBuf;
use std::fs;

#[test]
fn part1_prompt() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("inputs/prompt.txt");

    let prompt = fs::read_to_string(path).unwrap();

    assert_eq!(part1(prompt.to_string()), 6942);
}

#[test]
fn part2_prompt() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("inputs/prompt.txt");

    let prompt = fs::read_to_string(path).unwrap();

    assert_eq!(part2(prompt.to_string()), 297);
}

#[test]
fn part1_input1() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("inputs/input1.txt");

    let prompt = fs::read_to_string(path).unwrap();

    assert_eq!(part1(prompt.to_string()), 4);
}

#[test]
fn part1_input2() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("inputs/input2.txt");

    let prompt = fs::read_to_string(path).unwrap();

    assert_eq!(part1(prompt.to_string()), 8);
}

#[test]
fn part2_input3() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("inputs/input3.txt");

    let prompt = fs::read_to_string(path).unwrap();

    assert_eq!(part2(prompt.to_string()), 4);
}

#[test]
fn part2_input4() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("inputs/input4.txt");

    let prompt = fs::read_to_string(path).unwrap();

    assert_eq!(part2(prompt.to_string()), 8);
}

#[test]
fn part2_input5() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("inputs/input5.txt");

    let prompt = fs::read_to_string(path).unwrap();

    assert_eq!(part2(prompt.to_string()), 10);
}
