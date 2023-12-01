advent_of_code::solution!(1);

fn line_value(line: &str) -> Option<u32> {
    dbg!(line);

    let digits: Vec<char> = line.chars().filter(char::is_ascii_digit).collect();
    if digits.is_empty() {
        panic!("line did not have any digits: {}", line);
    }
    let first = digits.first()?.to_digit(10)?;
    let last = digits.last()?.to_digit(10)?;
    let val = (10 * first) + last;

    Some(val)
}

pub fn part_one(input: &str) -> Option<u32> {
    input.lines().map(line_value).sum()
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
        assert_eq!(result, Some(142));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
