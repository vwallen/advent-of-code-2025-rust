use std::rc::Rc;
use crate::read_input_lines;
use anyhow::Result;
use std::fmt::Debug;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Space {
    Empty,
    Roll,
    Bound,
}

#[derive(Debug)]
pub struct Warehouse {
    w: usize,
    h: usize,
    spaces: Vec<Vec<Space>>,
}
impl Warehouse {
    pub fn new(w:usize, h:usize) -> Warehouse {
        let mut spaces = vec![vec![Space::Bound; h + 2]; 1];
        for _ in 0..w {
            let mut col = vec![Space::Bound];
            col.append(&mut vec![Space::Empty; h]);
            col.push(Space::Bound);
            spaces.push(col);
        }
        spaces.push(vec![Space::Bound; h + 2]);
        Warehouse{ w, h, spaces }
    }

    pub fn place(&mut self, x:usize, y:usize, space:Space) {
        self.spaces[x][y] = space;
    }

    pub fn count_neighbors(&self, x:usize, y:usize) -> u8 {
        let mut count = 0;
        if self.spaces[x - 1][y]     == Space::Roll { count += 1 }
        if self.spaces[x - 1][y - 1] == Space::Roll { count += 1 }
        if self.spaces[x][y - 1]     == Space::Roll { count += 1 }
        if self.spaces[x + 1][y]     == Space::Roll { count += 1 }
        if self.spaces[x + 1][y + 1] == Space::Roll { count += 1 }
        if self.spaces[x][y + 1]     == Space::Roll { count += 1 }
        if self.spaces[x - 1][y + 1] == Space::Roll { count += 1 }
        if self.spaces[x + 1][y - 1] == Space::Roll { count += 1 }
        count
    }
}

pub fn prepare(file_name: &str) -> Result<Vec<String>> {
    let input = read_input_lines(file_name);
    Ok(input)
}

pub fn part_1(_input: &Vec<String>) -> Option<usize> {
    None
}

pub fn part_2(_input: &Vec<String>) -> Option<usize> {
    None
}

#[cfg(test)]
mod test {

    use super::*;
    use super::Space::*;

    #[test]
    fn test_warehouse() {
        let mut warehouse = Warehouse::new(2, 3);
        warehouse.place(2, 1, Space::Roll);
        warehouse.place(1, 2, Space::Roll);
        warehouse.place(2, 2, Space::Roll);
        assert_eq!(warehouse.spaces, vec![
            vec![Bound, Bound, Bound, Bound, Bound],
            vec![Bound, Empty, Roll,  Empty, Bound],
            vec![Bound, Roll,  Roll,  Empty, Bound],
            vec![Bound, Bound, Bound, Bound, Bound],
        ]);
        assert_eq!(warehouse.count_neighbors(2, 2), 2);
    }

    #[test]
    #[ignore]
    fn test_part_1() {
        if let Ok(input) = prepare("day04-example.txt") {
            assert_eq!(part_1(&input), Some(1))
        }
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        if let Ok(input) = prepare("day04-example.txt") {
            assert_eq!(part_2(&input), Some(1))
        }
    }
}