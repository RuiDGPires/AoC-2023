#![allow(dead_code, unused_imports)]
#[cfg(test)]
mod tests;
use regex::Regex;
use std::collections::HashMap;
use queues::*;

#[derive(Debug, Clone, Copy, Hash)]
struct Pos(i64, i64);

impl Pos {
    fn left(&self) -> Pos {
        Pos(self.0 - 1, self.1)
    }
    fn right(&self) -> Pos {
        Pos(self.0 + 1, self.1)
    }
    fn up(&self) -> Pos {
        Pos(self.0, self.1 - 1)
    }
    fn down(&self) -> Pos {
        Pos(self.0, self.1 + 1)
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 
    }
}
impl Eq for Pos {}


#[derive(Debug, Clone, Copy)]
struct Node {
    pos: Pos,
    parent: Pos,
    dist: usize,
}

impl Node {
    fn new(pos: Pos, parent: Pos, dist: usize) -> Self {
        Node{pos: pos, parent: parent, dist: dist}
    }

    fn from(pos: Pos, parent: &Node) -> Self {
        Node{pos: pos, parent: parent.pos, dist: parent.dist + 1}
    }
}

fn get_starting_pos<'a, T>(lines: T)  -> Option<Pos> where T: Iterator<Item=&'a str> {
    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                return Some(Pos(x as i64, y as i64));
            }
        }
    }
    None
}

fn get_cell(pos: &Pos, map: &Vec<Vec<char>>) -> Option<char> {
    if pos.0 < 0 || pos.1 < 0 {
        return None;
    }

    if let Some(row) = map.get(pos.1 as usize) {
        if let Some(c) = row.get(pos.0 as usize) {
            return Some(*c);
        }
    }

    None
}

fn is_connected_left(c: char) -> bool {
    match c {
        '-' | '7' | 'J' => true,
        _ => false,
    }
}

fn is_connected_right(c: char) -> bool {
    match c {
        '-' | 'F' | 'L' => true,
        _ => false,
    }
}

fn is_connected_up(c: char) -> bool {
    match c {
        '|' | 'J' | 'L' => true,
        _ => false,
    }
}

fn is_connected_down(c: char) -> bool {
    match c {
        '|' | '7' | 'F' => true,
        _ => false,
    }
}

fn get_neighbours(pos: &Pos, map: &Vec<Vec<char>>) -> Vec<Pos> {
    let mut ret = Vec::new();

    match get_cell(&pos, map).expect("Invalid cell") {
        '|' => {
            ret.push(pos.up());
            ret.push(pos.down());
        }
        '-' => {
            ret.push(pos.left());
            ret.push(pos.right());
        }
        'L' => {
            ret.push(pos.up());
            ret.push(pos.right());
        }

        'J' => {
            ret.push(pos.up());
            ret.push(pos.left());
        }
        '7' => {
            ret.push(pos.left());
            ret.push(pos.down());
        }
        'F' => {
            ret.push(pos.right());
            ret.push(pos.down());
        }
        'S' => {
            if let Some(c) = get_cell(&pos.up(), map) {
                if is_connected_down(c) {
                    ret.push(pos.up());
                }
            }
            if let Some(c) = get_cell(&pos.down(), map) {
                if is_connected_up(c) {
                    ret.push(pos.down());
                }
            }
            if let Some(c) = get_cell(&pos.left(), map) {
                if is_connected_right(c) {
                    ret.push(pos.left());
                }
            }
            if let Some(c) = get_cell(&pos.right(), map) {
                if is_connected_left(c) {
                    ret.push(pos.right());
                }
            }
        }
        _   => {}
    }

    ret.iter().filter(|pos| get_cell(pos, map).is_some())
        .map(|pos| *pos)
        .collect()
}

fn bfs(start: Pos, map: &Vec<Vec<char>>, hashmap: &mut HashMap<Pos, Node>) -> Node {
    let mut stack: Vec<Node> = Vec::new();

    stack.push(Node::new(start, start, 0));

    loop {
        let current = stack.pop().unwrap();

        if current.pos == start && current.dist > 3 {
            return current;
        }

        hashmap.entry(current.pos).or_insert_with(|| {
            for neighbour in get_neighbours(&current.pos, map) {
                stack.push(Node::from(neighbour, &current));
            }
            current
        });
    }
}

fn resolve_cell(pos: Pos, map: &Vec<Vec<char>>) -> char {
    let mut right = false;
    let mut left = false;
    let mut up = false;
    let mut down = false;

    if let Some(c) = get_cell(&pos.left(), map) {
        left = c != '.' && is_connected_right(c);
    }
    if let Some(c) = get_cell(&pos.right(), map) {
        right = c != '.' && is_connected_left(c);
    }
    if let Some(c) = get_cell(&pos.up(), map) {
        up = c != '.' && is_connected_down(c);
    }
    if let Some(c) = get_cell(&pos.down(), map) {
        down = c != '.' && is_connected_up(c);
    }

    if right && left {
        return '-';
    }
    if right && down {
        return 'F';
    }
    if right && up {
        return 'L';
    }
    if left && down {
        return '7';
    }
    if left && up {
        return 'J';
    }
    if up && down {
        return '|';
    }
    '.'
}

fn is_inside(pos: Pos, map: &Vec<Vec<char>>) -> bool {
    map[pos.1 as usize].iter().enumerate().filter(|(x, c)| {
        *x < pos.0 as usize &&
        (
            **c == '|' || **c == 'F' || **c == '7'
        )
    }).count() % 2 != 0
}

pub fn part1(prompt: String) -> u64 {
    let start = get_starting_pos(prompt.lines()).expect("Unable to get starting pos");
    let map: Vec<Vec<char>> = prompt
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut hashmap: HashMap<Pos, Node> = HashMap::new();

    let end = bfs(start, &map, &mut hashmap);

    return ((end.dist) / 2) as u64;
}

pub fn part2(prompt: String) -> u64 {
    let start = get_starting_pos(prompt.lines()).expect("Unable to get starting pos");
    let map: Vec<Vec<char>> = prompt
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut hashmap: HashMap<Pos, Node> = HashMap::new();

    let end = bfs(start, &map, &mut hashmap);

    let mut new_map: Vec<Vec<char>> = map.iter().map(|row| row.iter().map(|_| '.').collect()).collect();

    let mut current = end;

    while current.parent != start {
        new_map[current.pos.1 as usize][current.pos.0 as usize] = map[current.pos.1 as usize][current.pos.0 as usize];
        current = hashmap[&current.parent];
    }
    new_map[current.pos.1 as usize][current.pos.0 as usize] = map[current.pos.1 as usize][current.pos.0 as usize];
    new_map[start.1 as usize][start.0 as usize] = resolve_cell(start, &new_map);

    let mut cells = 0;
    for (y, row) in new_map.iter().enumerate() {
        for (x,c) in row.iter().enumerate() {
            if *c == '.' {
                cells += is_inside(Pos(x as i64, y as i64), &new_map) as u64;
            }
        }
    }

    cells
}
