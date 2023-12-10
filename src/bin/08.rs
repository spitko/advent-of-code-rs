use std::collections::HashMap;

use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

advent_of_code::solution!(8);

// key, left, right
// for next, match

pub fn part_one(input: &str) -> Option<u32> {
    let mut network: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut lines = input.lines();
    let directions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect_vec();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let re = Regex::new(r"\w+").unwrap();
        let mut matches = re.find_iter(line);
        let key = matches.next().unwrap().as_str();
        let left = matches.next().unwrap().as_str();
        let right = matches.next().unwrap().as_str();
        network.insert(key, (left, right));
    }
    let mut cur = network.get("AAA").unwrap();
    let mut steps = 0;
    loop {
        let (left, right) = cur;
        let next = match directions.get(steps % directions.len()).unwrap() {
            0 => left,
            1 => right,
            _ => panic!("Invalid direction"),
        };
        steps += 1;
        if next == &"ZZZ" {
            return Some(steps as u32);
        }
        cur = network.get(next).unwrap();
    }
}

#[derive(Debug)]
struct Node {
    key: String,
}

impl Node {
    pub fn end(&self) -> bool {
        return self.key.ends_with("Z");
    }

    pub fn advance(&mut self, map: &HashMap<&str, (String, String)>, direction: u8) {
        let (left, right) = map.get(self.key.as_str()).unwrap();
        let res = match direction {
            0 => left,
            1 => right,
            _ => panic!("Invalid direction"),
        };
        self.key = res.clone();
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut network: HashMap<&str, (String, String)> = HashMap::new();
    let mut starts: Vec<Node> = Vec::new();
    let mut lines = input.lines();
    let directions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect_vec();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let re = Regex::new(r"\w+").unwrap();
        let mut matches = re.find_iter(line);
        let key = matches.next().unwrap().as_str();
        if key.ends_with("A") {
            starts.push(Node {
                key: key.to_string(),
            });
        }
        let left = matches.next().unwrap().as_str();
        let right = matches.next().unwrap().as_str();
        network.insert(key, (String::from(left), String::from(right)));
    }
    let steps_to_end = starts
        .iter_mut()
        .map(|start| {
            let mut start_steps = 0;
            while !start.end() {
                start.advance(&network, directions[start_steps % directions.len()]);
                start_steps += 1;
            }
            return start_steps;
        })
        .collect_vec();
    dbg!(steps_to_end.as_slice());
    Some(lcm(steps_to_end.as_slice()) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}

pub fn lcm(nums: &[usize]) -> usize {
    nums.iter()
        .fold(1, |lcm, &num| lcm / gcd_of_two_numbers(lcm, num) * num)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
