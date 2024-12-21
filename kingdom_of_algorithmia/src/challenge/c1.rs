use crate::util;

use super::Challenge;

#[derive(Default)]
pub struct C1 {
    enemies_p1: Vec<u8>,
    enemies_p2: Vec<u8>,
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
        let lines = util::read_file_lines("src/input/c1p1.txt");
        self.enemies_p1 = lines[0].as_bytes().to_vec();

        let lines = util::read_file_lines("src/input/c1p2.txt");
        self.enemies_p2 = lines[0].as_bytes().to_vec();
    }

    fn part1(&self) -> String {
        let mut total = 0;
        for enemy in self.enemies_p1.iter() {
            match enemy {
                b'A' => {}
                b'B' => total += 1,
                b'C' => total += 3,
                _ => unreachable!(),
            }
        }
        format!("{total}")
    }

    fn part2(&self) -> String {
        let mut total = 0;
        let _enemies = "AxBCDDCAxD".as_bytes().to_vec();
        let enemies = &self.enemies_p2;
        for i in 0..enemies.len() / 2 {
            let mut pair_enemy = true;
            for offset in 0..=1 {
                match enemies[2*i+offset] {
                    b'A' => {}
                    b'B' => {
                        total += 1;
                    }
                    b'C' => {
                        total += 3;
                    }
                    b'D' => {
                        total += 5;
                    }
                    b'x' => pair_enemy = false,
                    _ => unreachable!(),
                }
            }
            
            if pair_enemy == true {
                total += 2;
            }
        }

        format!("{}", total)
    }

    fn part3(&self) -> String {
        "".to_string()
    }
}
