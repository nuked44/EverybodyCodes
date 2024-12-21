use crate::util;

use super::Challenge;

#[derive(Default)]
pub struct C2 {
    p1_words: Vec<String>,
    p1_text: Vec<String>,
}

impl C2 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Challenge for C2 {
    fn name(&self) -> String {
        format!("The Runes of Power")
    }

    fn parse_input(&mut self) {
        let lines = util::read_file_lines("src/input/c2p1.txt");
        self.p1_words = lines[0]
            .split_once(":")
            .unwrap()
            .1
            .split(",")
            .map(|s| s.to_string())
            .collect();
        self.p1_text = lines[2].split(" ").map(|s| s.to_string()).collect();
    }

    fn part1(&self) -> String {
        let text = &self.p1_text;
        let words = &self.p1_words;
        let mut total = 0;
        for text_part in text.iter() {
            for word in words.iter() {
                total += text_part.matches(word).count();
            }
        }
        format!("{total}")
    }

    fn part2(&self) -> String {
        format!("")
    }

    fn part3(&self) -> String {
        format!("")
    }
}
