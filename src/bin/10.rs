use itertools::Itertools;

advent_of_code::solution!(10);

type Direction = (isize, isize);

const DOWN: Direction = (0, 1);
const UP: Direction = (0, -1);
const RIGHT: Direction = (1, 0);
const LEFT: Direction = (-1, 0);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PipeType {
    Vertical,
    Horizontal,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    Ground,
    Start,
}

impl PipeType {
    pub fn from_char(c: char) -> Self {
        match c {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::UpRight,
            'J' => Self::UpLeft,
            'F' => Self::DownRight,
            '7' => Self::DownLeft,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("Invalid pipe"),
        }
    }

    pub fn next(&self, last_direction: (isize, isize)) -> (isize, isize) {
        match self {
            Self::Vertical => last_direction,
            Self::Horizontal => last_direction,
            Self::UpRight => match last_direction {
                DOWN => RIGHT,
                LEFT => UP,
                _ => panic!("Invalid direction"),
            },
            Self::UpLeft => match last_direction {
                DOWN => LEFT,
                RIGHT => UP,
                _ => panic!("Invalid direction"),
            },
            Self::DownRight => match last_direction {
                UP => RIGHT,
                LEFT => DOWN,
                _ => panic!("Invalid direction"),
            },
            Self::DownLeft => match last_direction {
                UP => LEFT,
                RIGHT => DOWN,
                _ => panic!("Invalid direction"),
            },
            Self::Ground => panic!("Invalid direction"),
            Self::Start => panic!("Invalid direction"),
        }
    }
}

fn find_possible_directions_to_go_from_start(
    grid: &Vec<Vec<PipeType>>,
    start: (isize, isize),
) -> Vec<(isize, isize)> {
    let mut res = vec![];
    let (x, y) = start;
    match grid[y as usize + 1][x as usize] {
        PipeType::Vertical | PipeType::DownLeft | PipeType::DownRight => res.push(DOWN),
        _ => {}
    };
    match grid[y as usize - 1][x as usize] {
        PipeType::Vertical | PipeType::UpLeft | PipeType::UpRight => res.push(UP),
        _ => {}
    };
    match grid[y as usize][x as usize + 1] {
        PipeType::Horizontal | PipeType::UpLeft | PipeType::DownLeft => res.push(RIGHT),
        _ => {}
    };
    if x > 0 {
        match grid[y as usize][x as usize - 1] {
            PipeType::Horizontal | PipeType::UpRight | PipeType::DownRight => res.push(LEFT),
            _ => {}
        };
    }
    res
}

fn advance(cur: (isize, isize), direction: (isize, isize)) -> (isize, isize) {
    (cur.0 + direction.0, cur.1 + direction.1)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|l| l.chars().map(PipeType::from_char).collect_vec())
        .collect_vec();
    let (s_y, s_x) = grid
        .iter()
        .flatten()
        .position(|c| c == &PipeType::Start)
        .map(|i| ((i / grid[0].len()) as isize, (i % grid[0].len()) as isize))
        .unwrap();

    let start_directions = find_possible_directions_to_go_from_start(&grid, (s_x, s_y));
    let mut cur: (isize, isize) = (s_x, s_y);
    let mut steps = 0;
    cur = advance(cur, start_directions[0]);
    let mut last_direction: (isize, isize) = start_directions[0];
    while cur != (s_x, s_y) {
        let pipe = grid[cur.1 as usize][cur.0 as usize];
        last_direction = pipe.next(last_direction);
        cur = advance(cur, last_direction);
        steps += 1;
    }
    Some(steps / 2 + 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
