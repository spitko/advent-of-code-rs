advent_of_code::solution!(6);

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn from_tuple(time_distance: (u64, u64)) -> Self {
        let (time, distance) = time_distance;
        Self { time, distance }
    }

    fn total_ways_to_win(&self) -> u32 {
        (1..=self.time)
            .filter(|&speed| (self.time - speed) * speed > self.distance)
            .count() as u32
    }
}

fn parse_number_line(line: &str) -> impl Iterator<Item = u64> + '_ {
    line.split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
}

fn parse_number_line_part_two(line: &str) -> u64 {
    line.split_whitespace()
        .skip(1)
        .fold(String::new(), |a: String, b| a + b)
        .parse::<u64>()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let times = parse_number_line(lines.next().unwrap());
    let distances = parse_number_line(lines.next().unwrap());
    let races = times.zip(distances).map(Race::from_tuple);
    races.fold(Some(1), |acc, elem| {
        Some(acc.unwrap() * elem.total_ways_to_win())
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let time = parse_number_line_part_two(lines.next().unwrap());
    let distance = parse_number_line_part_two(lines.next().unwrap());
    let race = Race { time, distance };
    Some(race.total_ways_to_win() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u32> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
