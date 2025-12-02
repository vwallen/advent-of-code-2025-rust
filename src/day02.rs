use crate::read_input;
use anyhow::Result;
use itertools::Itertools;

pub fn prepare(file_name: &str) -> Result<Vec<(u64, u64)>> {
    let input = read_input(file_name)
        .split(",")
        .map(|rng| {
            sscanf!(rng.trim(), "{u64}-{u64}").unwrap()
        })
        .collect();
    Ok(input)
}

pub fn is_double(id:u64) -> bool {
    let id_str = id.to_string();
    let parts = id_str.split_at(id_str.len() / 2);
    parts.0 == parts.1
}

pub fn is_repeated_sequence(id:u64) -> bool {
    let id_str = id.to_string();
    let max_chunk = id_str.len() / 2;

    (1..=max_chunk)
        .rev()
        .map(|chunk| {
            id_str
                .chars()
                .chunks(chunk)
                .into_iter()
                .map(|chunk| chunk.collect::<String>())
                .all_equal_value()
                .is_ok()
        })
        .any(|ok| ok)
}

pub fn part_1(input: &Vec<(u64, u64)>) -> Option<usize> {
    let total = input
        .iter()
        .fold(0, |mut total:usize, (a, b)| {
            for id in *a..=*b {
                if is_double(id) { total += id as usize  }
            }
            total
        });
    Some(total)
}

pub fn part_2(input: &Vec<(u64, u64)>) -> Option<usize> {
    let total = input
        .iter()
        .fold(0, |mut total:usize, (a, b)| {
            for id in *a..=*b {
                if is_repeated_sequence(id) { total += id as usize  }
            }
            total
        });
    Some(total)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_prepare() {
        let input = prepare("day02-example.txt").unwrap();
        assert_eq!(input.first().unwrap(), &(11, 22));
        assert_eq!(input.last().unwrap(), &(2121212118, 2121212124));
    }

    #[test]
    fn test_is_double() {
        assert_eq!(is_double(11), true);
        assert_eq!(is_double(13), false);
        assert_eq!(is_double(22), true);
        assert_eq!(is_double(123123), true);
        assert_eq!(is_double(1231230), false);
        assert_eq!(is_double(11231231), false);
        assert_eq!(is_double(111), false);
        assert_eq!(is_double(1111), true);
    }

    #[test]
    fn test_is_repeated_sequence() {
        assert_eq!(is_repeated_sequence(11), true);
        assert_eq!(is_repeated_sequence(111), true);
        assert_eq!(is_repeated_sequence(123123), true);
        assert_eq!(is_repeated_sequence(121212), true);
        assert_eq!(is_repeated_sequence(110), false);
        assert_eq!(is_repeated_sequence(1110), false);
        assert_eq!(is_repeated_sequence(12312312), false);
        assert_eq!(is_repeated_sequence(12121211), false);
    }

    #[test]
    fn test_part_1() {
        if let Ok(input) = prepare("day02-example.txt") {
            assert_eq!(part_1(&input), Some(1227775554))
        }
    }

    #[test]
    fn test_part_2() {
        if let Ok(input) = prepare("day02-example.txt") {
            assert_eq!(part_2(&input), Some(4174379265))
        }
    }
}