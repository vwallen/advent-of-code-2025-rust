use crate::read_input;
use anyhow::Result;

pub fn prepare(file_name: &str) -> Result<Vec<(u64, u64)>> {
    let input = read_input(file_name)
        .split(",")
        .map(|rng| {
            sscanf!(rng.trim(), "{u64}-{u64}").unwrap()
        })
        .collect();
    Ok(input)
}

pub fn part_1(input: &Vec<(u64, u64)>) -> Option<usize> {
    let total:u64 = input
        .iter()
        .map(|(a, b)| {
            (*a..=*b)
                .filter(|i| i % 2 == 0)
                .filter(|i| {
                    let id_str = i.to_string();
                    let parts = id_str.split_at(id_str.len() / 2);
                    parts.0 == parts.1
                }).sum::<u64>()
        })
        .sum();
    Some(total as usize)

}

pub fn part_2(input: &Vec<(u64, u64)>) -> Option<usize> {
    let total:u64 = input
        .iter()
        .map(|(a, b)| {
            (*a..=*b)
                .filter(|i| {
                    let id = i.to_string();
                    let id_len = id.len();
                    let id_bytes = id.as_bytes();
                    (1..=(id_len/2))
                        .filter(|c| id_len % c == 0)
                        .any(|c| {
                            let mut chunks = id_bytes.chunks(c);
                            let pattern = chunks.next();
                            chunks.all(|chunk| Some(chunk) == pattern)
                        })
                }).sum::<u64>()
        })
        .sum();
    Some(total as usize)
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