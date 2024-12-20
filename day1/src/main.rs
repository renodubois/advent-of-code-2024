use anyhow::Result;
use std::str::Lines;

fn main() -> Result<()> {
    let p1_res = part1(include_str!("input.txt").lines())?;
    let p2_res = part2(include_str!("input.txt").lines())?;

    println!("Part 1: {p1_res}");
    println!("Part 2: {p2_res}");
    Ok(())
}

fn part1(lines: Lines) -> Result<usize> {
    let mut total = 0;
    let mut list1 : Vec<usize> = Vec::new();
    let mut list2 : Vec<usize> = Vec::new();
    for line in lines {
        for (i, val) in line.split_whitespace().enumerate() {
            let parsed_val = val.parse::<usize>().unwrap();
            if i == 0 {
                list1.push(parsed_val);
            } else {
                list2.push(parsed_val);
            }
        }
    }

    list1.sort();
    list2.sort();

    for (i, val) in list1.iter().enumerate() {
        let l2 = list2[i];
        if *val > l2 {
            total += val - l2;
        } else {
            total += l2 - val;
        }
    }

    Ok(total)
}

fn part2(lines: Lines) -> Result<usize> {
    let mut total = 0;
    let mut list1 : Vec<usize> = Vec::new();
    let mut list2 : Vec<usize> = Vec::new();
    for line in lines {
        for (i, val) in line.split_whitespace().enumerate() {
            let parsed_val = val.parse::<usize>().unwrap();
            if i == 0 {
                list1.push(parsed_val);
            } else {
                list2.push(parsed_val);
            }
        }
    }

    for (i, val) in list1.iter().enumerate() {
        let mut sim = 0;
        for val2 in list2.iter() {
            if val == val2 {
                sim += 1;
            }
        }

        total += val * sim;
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(include_str!("pt1_example.txt").lines())?, 11);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(include_str!("pt2_example.txt").lines())?, 31);
        Ok(())
    }
}
