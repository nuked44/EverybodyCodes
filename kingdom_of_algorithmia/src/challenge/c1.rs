use std::hint::black_box;

use crate::util;

use super::Challenge;

#[derive(Default)]
pub struct C1 {
    enemies: Vec<u8>
}

impl C1 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Challenge for C1 {
    fn name(&self) -> String {
        "The Battle for the Farmlands".to_string()
    }

    fn parse_input(&mut self) {
        let lines = util::read_file_lines("src/input/c1.txt");
        self.enemies = lines[0].as_bytes().to_vec();
    }

    fn part1(&self) -> String {
        let mut total = 0;
        for enemy in self.enemies.clone() {
            match enemy {
                b'A' => {},
                b'B' => total += 1,
                b'C' => total += 3,
                _ => unreachable!(),
            }
        }
        format!("{total}")
    }

    fn part2(&self) -> String {
        "".to_string()
    }

    fn part3(&self) -> String {
        "".to_string()
    }
}