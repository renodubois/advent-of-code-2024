use anyhow::Result;
use std::str::Lines;

fn main() -> Result<()> {
    let p1_res = part1(include_str!("input.txt").lines())?;
    let p2_res = part2(include_str!("input.txt").lines())?;

    println!("Part 1: {p1_res}");
    println!("Part 2: {p2_res}");
    Ok(())
}

#[derive(PartialEq)]
enum Behavior {
    Unset = 0,
    Asc = 1,
    Desc = 2,
}

fn part1(lines: Lines) -> Result<usize> {
    let mut total = 0;

    for line in lines {
        let mut prev_val = 0;
        let mut safe = true;
        // array behavior: 0 is unset, 1 is asc, 2 is desc
        let mut behavior = Behavior::Unset;
        for val in line.split_whitespace() {
            let parsed_val = val.parse::<usize>().unwrap();
            if prev_val == 0 {
                prev_val = parsed_val;
            } else {
                if prev_val == parsed_val
                    || (prev_val > parsed_val && behavior == Behavior::Asc)
                    || (prev_val < parsed_val && behavior == Behavior::Desc)
                {
                    safe = false;
                    break;
                } else {
                    let mut diff = 0;
                    if parsed_val > prev_val {
                        behavior = Behavior::Asc;
                        diff = parsed_val - prev_val;
                    } else if prev_val > parsed_val {
                        behavior = Behavior::Desc;
                        diff = prev_val - parsed_val;
                    }
                    if diff > 3 {
                        safe = false;
                        break;
                    }
                }
                prev_val = parsed_val;
            }
        }

        if safe == true {
            total += 1
        }
    }

    Ok(total)
}

fn parse_line(line: Vec<usize>) -> bool {
    let mut prev_val = 0;
    let mut behavior = Behavior::Unset;

    for val in line {
        if prev_val == 0 {
            prev_val = val;
        } else {
            if prev_val == val
                || (prev_val > val && behavior == Behavior::Asc)
                || (prev_val < val && behavior == Behavior::Desc)
            {
                return false;
            } else {
                let mut diff = 0;
                if val > prev_val {
                    if behavior == Behavior::Unset {
                        behavior = Behavior::Asc;
                    }
                    diff = val - prev_val;
                } else if prev_val > val {
                    if behavior == Behavior::Unset {
                        behavior = Behavior::Desc;
                    }
                    diff = prev_val - val;
                }
                if diff > 3 {
                    return false;
                }
            }

            prev_val = val;
        }
    }
    true
}

fn part2(lines: Lines) -> Result<usize> {
    let mut total = 0;

    // performance who?

    for line in lines {
        let parsed_line: Vec<usize> = line
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let mut safe = parse_line(parsed_line.clone());
        if !safe {
            for (i, _) in parsed_line.iter().enumerate() {
                let mut line_copy = parsed_line.clone();
                line_copy.remove(i);
                if parse_line(line_copy) {
                    safe = true;
                    break;
                }
            }
        }

        if safe == true {
            total += 1
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(include_str!("pt1_example.txt").lines())?, 2);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(include_str!("pt2_example.txt").lines())?, 1);
        Ok(())
    }
}
