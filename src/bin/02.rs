use std::cmp::max;

use sscanf::sscanf;

advent_of_code::solution!(2);
// use sscanf::sscanf;

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Blue,
    Green,
}

#[derive(Debug)]
struct Draw {
    count: usize,
    color: Color,
}

#[derive(Debug)]
struct Round {
    draws: Vec<Draw>,
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

fn parse_color(color_str: &str) -> Color {
    match color_str {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        _ => panic!("Invalid Color: {}", color_str),
    }
}

fn parse_draw(draw_str: &str) -> Draw {
    let (count, color_str) = sscanf!(draw_str, "{usize} {&str}").unwrap();
    let color = parse_color(color_str);
    Draw { count, color }
}

fn parse_round(round_str: &str) -> Round {
    let draws: Vec<Draw> = round_str.split(", ").map(parse_draw).collect();
    Round { draws }
}

fn parse_game(line: &str) -> Game {
    let (id, rounds_str) = sscanf!(line, "Game {usize}: {&str}").unwrap();
    let rounds: Vec<Round> = rounds_str.split("; ").map(parse_round).collect();

    Game { id, rounds }
}

fn cubes_required(game: &Game) -> (usize, usize, usize) {
    let mut required_red = 0;
    let mut required_green = 0;
    let mut required_blue = 0;

    for rounds in game.rounds.iter() {
        for draw in rounds.draws.iter() {
            if draw.color == Color::Red {
                required_red = max(required_red, draw.count);
            } else if draw.color == Color::Green {
                required_green = max(required_green, draw.count);
            } else if draw.color == Color::Blue {
                required_blue = max(required_blue, draw.count);
            }
        }
    }
    (required_red, required_green, required_blue)
}

fn is_possible(game: &Game, query_red: usize, query_blue: usize, query_green: usize) -> usize {
    let (required_red, required_green, required_blue) = cubes_required(game);

    if required_red <= query_red && required_green <= query_green && required_blue <= query_blue {
        game.id
    } else {
        0
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let query_red = 12;
    let query_green = 13;
    let query_blue = 14;

    Some(
        input
            .lines()
            .map(parse_game)
            .map(|game| is_possible(&game, query_red, query_blue, query_green))
            .sum::<usize>()
            .try_into()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(parse_game)
            .map(|g| cubes_required(&g))
            .map(|(r, g, b)| r * g * b)
            .sum::<usize>()
            .try_into()
            .unwrap(),
    )
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
        assert_eq!(result, Some(2286));
    }
}
