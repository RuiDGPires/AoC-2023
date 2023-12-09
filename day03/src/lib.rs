#![allow(dead_code, unused_imports)]
mod tests;

#[derive(Clone, Copy, Debug)]
struct Number {
    value: u64,
    valid: bool
}

impl Number {
    fn new(val: u64) -> Self {
        Self {value: val, valid: false}
    }
}

fn part1(prompt: String) -> u64 {
    let mut matrix: Vec<Vec<u32>> = Vec::new();
    let mut nums: Vec<Number> = Vec::new();

    let mut number   = "".to_string();
    let mut number_n = 1 as u32; // 1 indexed

    for row in prompt.split("\n") {
        let mut row_vec: Vec<u32> = Vec::new();

        for c in row.chars() {
            if c >= '0' && c <= '9' {
                row_vec.push(number_n);
                number.push(c);
            } else {
                if number != "" {
                    nums.push(Number::new(number.parse::<u64>().unwrap()));
                    number_n += 1;
                    number = "".to_string();
                }
                row_vec.push(0)
            }
        } 
        if number != "" {
            nums.push(Number::new(number.parse::<u64>().unwrap()));
            number_n += 1;
            number = "".to_string();
        }
        matrix.push(row_vec);
    }

    let mut x = 0 as i32;
    let mut y = 0 as i32;

    for row in prompt.split("\n") {
        for c in row.chars() {
            if (c < '0' || c > '9') && c != '.' {
                for i in -1 .. 2 {
                    for j in -1 .. 2 {
                        if let Some(m_row) = matrix.get((y + i) as usize) {
                            if let Some(num) = m_row.get((x + j) as usize) {
                                if *num > 0 {
                                    nums[(*num as i32 -1) as usize].valid = true; 
                                }
                            }
                        }
                    }
                }
            }
            x += 1;
        }
        y += 1;
        x = 0;
    }

    //println!("{:?}", nums.iter().filter(|&item| item.valid).collect::<Vec<&Number>>());

    nums.iter()
        .filter(|&item| item.valid)
        .map(|&item| item.value)
        .sum::<u64>()

}

fn part2(prompt: String) -> u64 {
    let mut matrix: Vec<Vec<u32>> = Vec::new();
    let mut nums: Vec<Number> = Vec::new();

    let mut number   = "".to_string();
    let mut number_n = 1 as u32; // 1 indexed

    for row in prompt.split("\n") {
        let mut row_vec: Vec<u32> = Vec::new();

        for c in row.chars() {
            if c >= '0' && c <= '9' {
                row_vec.push(number_n);
                number.push(c);
            } else {
                if number != "" {
                    nums.push(Number::new(number.parse::<u64>().unwrap()));
                    number_n += 1;
                    number = "".to_string();
                }
                row_vec.push(0)
            }
        } 
        if number != "" {
            nums.push(Number::new(number.parse::<u64>().unwrap()));
            number_n += 1;
            number = "".to_string();
        }
        matrix.push(row_vec);
    }

    let mut x = 0 as i32;
    let mut y = 0 as i32;
    let mut total = 0 as u64;

    for row in prompt.split("\n") {
        for c in row.chars() {
            if c == '*' {
                let mut adjacent = vec![0, 0];
                let mut adjacent_n = 0 as usize;
                for i in -1 .. 2 {
                    for j in -1 .. 2 {
                        if let Some(m_row) = matrix.get((y + i) as usize) {
                            if let Some(num) = m_row.get((x + j) as usize) {
                                if *num > 0 {
                                    if adjacent_n >= 2 {
                                        break;
                                    }

                                    let current = nums.get_mut((*num as i32 -1) as usize).unwrap();
                                    if !current.valid {
                                        current.valid = true;
                                        adjacent[adjacent_n] = current.value;
                                        adjacent_n += 1;
                                    }
                                }
                            }
                        }
                    }
                }

                if adjacent_n == 2 {
                    total += adjacent[0] * adjacent[1];
                }
            }
            x += 1;
        }
        y += 1;
        x = 0;
    }

    total
}
