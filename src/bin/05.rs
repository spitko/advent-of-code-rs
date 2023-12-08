use rayon::prelude::*;
use std::{collections::HashMap, ops::Range};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let mut seeds: Option<Vec<u64>> = None;
    let mut mappings: [HashMap<Range<u64>, i64>; 7] = Default::default();

    let mut layer = 0;
    while let Some(line) = lines.next() {
        match line {
            line if line.starts_with("seeds:") => {
                let res = line
                    .split_whitespace()
                    .skip(1)
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect();
                seeds = Some(res);
            }
            line if line.ends_with("map:") => {
                while let Some(line) = lines.next() {
                    if line.is_empty() {
                        break;
                    }
                    let mut split = line.split_whitespace().map(|s| s.parse::<u64>().unwrap());
                    let destination: u64 = split.next().unwrap();
                    let source = split.next().unwrap();
                    let length = split.next().unwrap();
                    mappings[layer].insert(
                        source..source + length,
                        i64::try_from(destination).unwrap() - i64::try_from(source).unwrap(),
                    );
                }
                layer += 1;
            }
            _ => {}
        }
    }
    seeds
        .unwrap()
        .iter()
        .map(|seed| {
            let mut value = *seed;
            let mut layer = 0;
            while layer < 7 {
                value = mappings[layer]
                    .iter()
                    .find(|&(key, _)| key.contains(&value))
                    .map_or(value, |(_, &offset)| {
                        (i64::try_from(value).unwrap() + offset).try_into().unwrap()
                    });
                layer += 1;
            }
            value
        })
        .min()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let mut seeds: Option<Vec<(u64, u64)>> = None;
    let mut mappings: [HashMap<Range<u64>, i64>; 7] = Default::default();

    let mut layer = 0;
    while let Some(line) = lines.next() {
        match line {
            line if line.starts_with("seeds:") => {
                let iter = line
                    .split_whitespace()
                    .skip(1)
                    .map(|s| s.parse::<u64>().unwrap());
                seeds = Some(
                    iter.collect::<Vec<u64>>()
                        .chunks(2)
                        .filter_map(|chunk| match *chunk {
                            [a, b] => Some((a, b)),
                            _ => None,
                        })
                        .collect(),
                );
            }
            line if line.ends_with("map:") => {
                while let Some(line) = lines.next() {
                    if line.is_empty() {
                        break;
                    }
                    let mut split = line.split_whitespace().map(|s| s.parse::<u64>().unwrap());
                    let destination = split.next().unwrap();
                    let source = split.next().unwrap();
                    let length = split.next().unwrap();
                    mappings[layer].insert(
                        source..source + length,
                        i64::try_from(destination).unwrap() - i64::try_from(source).unwrap(),
                    );
                }
                layer += 1;
            }
            _ => {}
        }
    }
    seeds
        .unwrap()
        .par_iter()
        .flat_map(|(seed, count)| {
            let mut results: Vec<u64> = vec![];
            let value = *seed;
            for mut val in value..value + count {
                let mut layer: usize = 0;
                while layer < 7 {
                    val = mappings[layer]
                        .iter()
                        .find(|&(key, _)| key.contains(&val))
                        .map_or(val, |(_, &offset)| {
                            (i64::try_from(val).unwrap() + offset).try_into().unwrap()
                        });
                    layer += 1;
                }
                results.push(val);
            }
            results
        })
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
