advent_of_code::solution!(1);

#[macro_use]
extern crate lazy_static;

use std::{collections::HashMap, u128::MAX};

lazy_static! {
    static ref WORD_VALUES: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();

        m.insert("one", 1);
        m.insert("two", 2);
        m.insert("three", 3);
        m.insert("four", 4);
        m.insert("five", 5);
        m.insert("six", 6);
        m.insert("seven", 7);
        m.insert("eight", 8);
        m.insert("nine", 9);

        m
    };
}

fn line_value_digits(line: &str) -> Option<u32> {
    let digits: Vec<char> = line.chars().filter(char::is_ascii_digit).collect();
    if digits.is_empty() {
        panic!("line did not have any digits: {}", line);
    }
    let first = digits.first()?.to_digit(10)?;
    let last = digits.last()?.to_digit(10)?;
    let val = (10 * first) + last;

    Some(val)
}

fn get_word_value(current_word: &str) -> Option<u32> {
    if let Ok(digit) = current_word.parse::<u32>() {
        return Some(digit);
    }

    WORD_VALUES.get(current_word).copied()
}

// No number word is longer than five.
const MAX_WORD_LENGTH: usize = 5;

fn line_value_words(line: &str) -> Option<u32> {
    let mut digits: Vec<u32> = vec![];

    for i in 0..line.len() {
        for j in i..=line.len() {
            let slice = &line[i..j];
            if slice.len() > MAX_WORD_LENGTH {
                continue;
            }
            if let Some(val) = get_word_value(slice) {
                digits.push(val);
            }
        }
    }

    let first = digits.first()?;
    let last = digits.last()?;
    let val = (10 * first) + last;
    Some(val)
}

pub fn part_one(input: &str) -> Option<u32> {
    input.lines().map(line_value_digits).sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    input.lines().map(line_value_words).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two_example_two() {
        // Words May Overlap
        let example = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let result = part_two(example);
        assert_eq!(result, Some(281));
    }
}
