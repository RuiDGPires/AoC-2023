#![allow(dead_code, unused_imports)]
#[cfg(test)]
mod tests;

const SIZE: u64 = 1_000_000;

pub fn part1(prompt: String) -> u64 {
    let empty_rows: Vec<u64> = prompt
                    .lines()
                    .enumerate()
                    .filter(|(_, line)| line.chars().filter(|c| c != &'.').count() == 0)
                    .map(|(i, _)| i as u64)
                    .collect()
                    ;
    let empty_cols: Vec<u64> = {
        let mut iter = prompt.lines();
        let mut cols: Vec<char> = iter.next().unwrap().chars().collect();
        while let Some(current) = iter.next() {
            for (x, c) in current.chars().enumerate() {
                if c != '.' {
                   cols[x] = '#';
                }
            }
        }

        cols.iter().enumerate().filter(|(_, c)| **c == '.').map(|(i, _)| i as u64).collect()
    };

    let mut map: Vec<Vec<char>> = prompt.lines()
                                    .map(|line| line.chars().collect())
                                    .collect();
    let row_len = map.len();

    for (off, i) in empty_rows.iter().enumerate() {
        map.insert(*i as usize + off , vec!['.'; row_len])
    }

    for (off, i) in empty_cols.iter().enumerate() {
        for x in 0..row_len + empty_rows.len(){
            map[x].insert(*i as usize + off , '.');
        }
    }

    let points: Vec<(u64, u64)> = map.iter()
                    .enumerate()
                    .map(|(y, line)| line
                                    .iter()
                                    .enumerate()
                                    .filter(|(_, c)| **c == '#')
                                    .map(|(i, _)| (i as u64, y as u64))
                                    .collect::<Vec<(u64, u64)>>()
                        )
                    .filter(|line| !line.is_empty())
                    .fold(Vec::new(), |mut v, mut l| {v.append(&mut l); v})
                    ;

    let len = points.len();
    let mut sum = 0;
    for i in 0..len {
        for j in (i+1)..len {
            sum += (points[i].0 as i64 - points[j].0 as i64).abs() + (points[i].1 as i64 - points[j].1 as i64).abs();
        }
    }
    //for row in map {
    //    for c in row {
    //        print!("{}", c);
    //    }
    //    print!("\n");
    //}
    //println!("{:?}", points);

    sum as u64
}
                    

pub fn part2(prompt: String) -> u64 {
    let empty_rows: Vec<u64> = prompt
                    .lines()
                    .enumerate()
                    .filter(|(_, line)| line.chars().filter(|c| c != &'.').count() == 0)
                    .map(|(i, _)| i as u64)
                    .collect()
                    ;
    let empty_cols: Vec<u64> = {
        let mut iter = prompt.lines();
        let mut cols: Vec<char> = iter.next().unwrap().chars().collect();
        while let Some(current) = iter.next() {
            for (x, c) in current.chars().enumerate() {
                if c != '.' {
                   cols[x] = '#';
                }
            }
        }

        cols.iter().enumerate().filter(|(_, c)| **c == '.').map(|(i, _)| i as u64).collect()
    };

    let map: Vec<Vec<char>> = prompt.lines()
                                    .map(|line| line.chars().collect())
                                    .collect();

    let points: Vec<(u64, u64)> = map.iter()
                    .enumerate()
                    .map(|(y, line)| line
                                    .iter()
                                    .enumerate()
                                    .filter(|(_, c)| **c == '#')
                                    .map(|(i, _)| (i as u64, y as u64))
                                    .collect::<Vec<(u64, u64)>>()
                        )
                    .filter(|line| !line.is_empty())
                    .fold(Vec::new(), |mut v, mut l| {v.append(&mut l); v})
                    ;

    let len = points.len();
    let mut sum = 0;
    for i in 0..len {
        for j in (i+1)..len {
            sum += {
                let p1 = points[i];
                let p2 = points[j];
                let mut dist = 0; 
                for x in p1.0.min(p2.0) .. p1.0.max(p2.0) {
                    if empty_cols.contains(&x) {
                        dist += SIZE;
                    } else {
                        dist += 1;
                    }
                }
                for y in p1.1.min(p2.1) .. p1.1.max(p2.1) {
                    if empty_rows.contains(&y) {
                        dist += SIZE;
                    } else {
                        dist += 1;
                    }
                }

                dist
            }
        }
    }
    //for row in map {
    //    for c in row {
    //        print!("{}", c);
    //    }
    //    print!("\n");
    //}
    //println!("{:?}", points);

    sum as u64
}
