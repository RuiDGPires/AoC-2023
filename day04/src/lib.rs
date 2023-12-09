#![allow(dead_code, unused_imports)]
mod tests;

fn part1(prompt: String) -> u32 {
    let mut total = 0;

    for line in prompt.lines() {
        total += {
            let parts = line.split("|").collect::<Vec<&str>>();
            let win = parts[0].split(":").collect::<Vec<&str>>()[1].split(" ").collect::<Vec<&str>>();
            let nums = parts[1].split(" ");

            let line_total = nums.filter(|x| x != &"").fold(0, |sum, y| sum +  win.contains(&y) as u32);

            match line_total {
                0 => 0,
                x => u32::pow(2, x - 1),
            }
        }
    }

    total
}

fn part2(prompt: String) -> u32 {
    let size = prompt.lines().count();
    let mut copies = vec![1 as u32; size];

    for (i, line) in prompt.lines().enumerate() {
        let matches =  {
            let parts = line.split("|").collect::<Vec<&str>>();
            let win = parts[0].split(":").collect::<Vec<&str>>()[1].split(" ").collect::<Vec<&str>>();
            let nums = parts[1].split(" ");

            nums.filter(|x| x != &"").fold(0, |sum, y| sum +  win.contains(&y) as usize)
        };

        for j in (0..usize::min(matches, size)).map(|x| x + i + 1)  {
            copies[j] += copies[i];
        }
    }

    copies.iter().sum()
}
