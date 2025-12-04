use rustc_hash::FxHashMap;
use crate::read_input_lines;
use anyhow::Result;

static MAX_NEIGHBORS:u8 = 4;

type Warehouse = FxHashMap<(isize, isize), u8>;

pub fn prepare(file_name: &str) -> Result<Warehouse> {
    let input = read_input_lines(file_name);
    let output = input
        .iter()
        .enumerate()
        .map(|(y, line)| {
            let y = y as isize;
            line
                .chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    let x = x as isize;
                    if c == '@' {
                        Some((x, y, 0))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(isize, isize, u8)>>()
        })
        .fold(Warehouse::default(), |mut out, row| {
            row.into_iter()
                .for_each(|(x, y, s)| {
                    out.insert((x, y), s);
                });
            out
        });
    Ok(update_neighbors(&output))
}

pub fn update_neighbors(spaces:&Warehouse) -> Warehouse {
    let mut new_spaces = Warehouse::default();
    for ((x, y), _) in spaces.into_iter() {
        let (x, y) = (*x, *y);
        let count:u8 = [
            (x - 1, y),
            (x - 1, y - 1),
            (x - 1, y + 1),
            (x + 1, y),
            (x + 1, y + 1),
            (x + 1, y - 1),
            (x, y + 1),
            (x, y - 1),
        ]
            .into_iter()
            .fold(0, |count, (x, y)| {
                if let Some(_) = spaces.get(&(x, y)) {
                    count + 1
                } else {
                    count
                }
        });
        new_spaces.insert((x, y), count);
    };
    new_spaces
}

pub fn remove_rolls(spaces:&Warehouse) -> (usize, Warehouse) {
    let mut new_spaces = Warehouse::default();
    let mut removed = 0;
    for ((x, y), space) in spaces.into_iter() {
        let (x, y) = (*x, *y);
        if *space < MAX_NEIGHBORS {
            removed += 1;
        } else {
            new_spaces.insert((x, y), *space);
        }
    };
    (removed, update_neighbors(&new_spaces))
}

// Answer: 1349
pub fn part_1(input: &Warehouse) -> Option<usize> {
    let count = input
        .clone()
        .into_values()
        .filter_map(|space| {
            if space < MAX_NEIGHBORS {
                Some(space)
            } else {
                None
            }
        }).count();
    Some(count)
}

// Answer: 8277
pub fn part_2(input: &Warehouse) -> Option<usize> {
    let mut total_removed = 0;
    let mut removed = usize::MAX;
    let mut spaces = input.clone();
    while removed > 0 {
        (removed, spaces) = remove_rolls(&spaces);
        total_removed += removed;
    };
    Some(total_removed)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_prepare() {
        if let Ok(input) = prepare("day04-example.txt") {
            // . . 3 @ . @ @ @ @ . <---- (2, 0)
            // @ @ @ . @ . @ . @ @
            // @ @ @ @ @ . @ . @ @
            // @ . @ @ @ @ . . @ .
            // @ @ . @ 8 @ @ . @ @ <---- (4, 4)
            // . @ @ @ @ @ @ @ . @
            // . @ . @ . @ . @ @ @
            // @ . @ @ @ . @ @ @ @
            // . @ @ @ @ @ @ @ @ 4 <---- (9, 8)
            // @ . @ . @ @ @ . @ .
            let input = update_neighbors(&input);
            assert_eq!(input.get(&(0, 0)), None);
            assert_ne!(input.get(&(2, 0)), None);
            assert_ne!(input.get(&(8, 0)), None);
            assert_eq!(input.get(&(9, 9)), None);
            if let Some(neighbors) = input.get(&(2, 0)) {
                assert_eq!(*neighbors, 3);
            }
            if let Some(neighbors) = input.get(&(4, 4)) {
                assert_eq!(*neighbors, 8);
            }
            if let Some(neighbors) = input.get(&(9, 8)) {
                assert_eq!(*neighbors, 4);
            }
        }
    }

    #[test]
    fn test_part_1() {
        if let Ok(input) = prepare("day04-example.txt") {
            assert_eq!(part_1(&input), Some(13))
        }
    }

    #[test]
    fn test_part_2() {
        if let Ok(input) = prepare("day04-example.txt") {
            assert_eq!(part_2(&input), Some(43))
        }
    }
}