advent_of_code::solution!(3);

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let height = grid.len();
    let width = grid.first()?.len();
    let mut visited = vec![vec![false; width]; height];
    Some(
        (0..grid.len())
            .cartesian_product(0..grid.first()?.len())
            .filter_map(|(i, j)| match part_one_dfs(&grid, &mut visited, i, j, height, width, 0) {
                Some((true, val)) => Some(val),
                _ => None,
            })
            .sum(),
    )
}

fn part_one_dfs(
    grid: &[Vec<char>],
    visited: &mut [Vec<bool>],
    i: usize,
    j: usize,
    height: usize,
    width: usize,
    last_val: u32,
) -> Option<(bool, u32)> {
    if j < width {
        if visited[i][j] {
            return None;
        }
        visited[i][j] = true;
    }

    // We've computed the full number and can return the accumulated value.
    if j >= width || grid[i][j] == '.' {
        Some((false, last_val))
    } else if !grid[i][j].is_ascii_digit() {
        Some((true, last_val))
    } else {
        let adjacent = is_adjacent_to_symbol(grid, i, j, height, width);
        let current_val = (10 * last_val) + grid[i][j].to_digit(10).unwrap();
        let (adj, full_val) = part_one_dfs(grid, visited, i, j + 1, height, width, current_val)?;
        Some((adj || adjacent, full_val))
    }
}

// Returns true if square (`i`, `j`) is adjacent to a symbol in `grid`.
// `height` and `width` are used to perform bounds checking.
fn is_adjacent_to_symbol(
    grid: &[Vec<char>],
    i: usize,
    j: usize,
    height: usize,
    width: usize,
) -> bool {
    const DELTAS: [isize; 3] = [-1, 0, 1];
    DELTAS
        .iter()
        .cartesian_product(DELTAS)
        .filter_map(|(dx, dy)| {
            let a = i.checked_add_signed(*dx)?;
            let b = j.checked_add_signed(dy)?;
            if a < height && b < width {
                Some((a, b))
            } else {
                None
            }
        })
        .any(|(a, b)| !grid[a][b].is_ascii_digit() && grid[a][b] != '.')
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let height = grid.len();
    let width = grid.first()?.len();

    Some(
        (0..grid.len())
            .cartesian_product(0..grid.first()?.len())
            .filter_map(|(i, j)| get_numbers(&grid, i, j, height, width))
            .filter(|v| v.len() == 2)
            .filter_map(|v| v.iter().copied().reduce(|a, b| a * b))
            .sum::<u32>(),
    )
}

fn get_numbers(
    grid: &[Vec<char>],
    i: usize,
    j: usize,
    height: usize,
    width: usize,
) -> Option<Vec<u32>> {
    if grid[i][j] != '*' {
        return None;
    }
    let mut nums: Vec<u32> = Vec::new();
    if let Some(right) = get_right(grid, i, j + 1, width) {
        nums.push(to_u32(right));
    }
    if let Some(left) = get_left(grid, i, (j as isize) - 1) {
        nums.push(to_u32(left));
    }

    // Look Up
    if ((i as isize) - 1) >= 0 {
        let right = get_right(grid, i - 1, j + 1, width);
        let middle = get_middle(grid[i - 1][j]);
        let left = get_left(grid, i - 1, (j as isize) - 1);
        create_number(left, middle, right, &mut nums);
    }

    // Look Down.
    if (i + 1) < height {
        let right = get_right(grid, i + 1, j + 1, width);
        let middle = get_middle(grid[i + 1][j]);
        let left = get_left(grid, i + 1, (j as isize) - 1);
        create_number(left, middle, right, &mut nums);
    }

    Some(nums)
}

fn create_number(
    left: Option<Vec<char>>,
    middle: Option<Vec<char>>,
    right: Option<Vec<char>>,
    nums: &mut Vec<u32>,
) {
    match (left, middle, right) {
        (None, None, None) => (),
        (None, None, Some(r)) => nums.push(to_u32(r)),
        (None, Some(mut m), Some(r)) => {
            m.extend(r);
            nums.push(to_u32(m));
        }
        (Some(l), None, None) => nums.push(to_u32(l)),
        (Some(mut l), Some(m), None) => {
            l.extend(m);
            nums.push(to_u32(l));
        }
        (Some(l), None, Some(r)) => {
            nums.push(to_u32(l));
            nums.push(to_u32(r));
        }
        (Some(mut l), Some(m), Some(r)) => {
            l.extend(m);
            l.extend(r);
            nums.push(to_u32(l));
        }
        (None, Some(m), None) => nums.push(to_u32(m)),
    }
}

fn to_u32(v: Vec<char>) -> u32 {
    v.into_iter().collect::<String>().parse().unwrap()
}

fn get_right(grid: &[Vec<char>], i: usize, j: usize, width: usize) -> Option<Vec<char>> {
    if j >= width || !grid[i][j].is_ascii_digit() {
        None
    } else {
        Some((j..width).map(|y| grid[i][y]).take_while(|x| x.is_ascii_digit()).collect())
    }
}

fn get_middle(x: char) -> Option<Vec<char>> {
    if x.is_ascii_digit() {
        Some(vec![x])
    } else {
        None
    }
}

fn get_left(grid: &[Vec<char>], i: usize, j: isize) -> Option<Vec<char>> {
    if j <= 0 || !grid[i][j as usize].is_ascii_digit() {
        None
    } else {
        let mut flipped = (0..=j)
            .rev()
            .map(|y| grid[i][y as usize])
            .take_while(|x| x.is_ascii_digit())
            .collect::<Vec<char>>();
        flipped.reverse();
        Some(flipped)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
