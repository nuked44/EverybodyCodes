use std::collections::HashSet;

use crate::util;

use super::Challenge;

#[derive(Default)]
pub struct C2 {
    p1_words: Vec<String>,
    p1_text: Vec<String>,
    p2_words: Vec<String>,
    p2_text: Vec<String>,
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

        let lines = util::read_file_lines("src/input/c2p2.txt");
        self.p2_words = lines[0]
            .split_once(":")
            .unwrap()
            .1
            .split(",")
            .map(|s| s.to_string())
            .collect();
        self.p2_text = lines[2..].to_vec();
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

        let text = &self.p2_text;
        let mut words = self.p2_words.clone();
        let mut rev_words = words
            .iter()
            .map(|w| w.chars().rev().collect::<String>())
            .collect::<Vec<String>>();
        words.append(&mut rev_words);

        let mut total = 0;
        for text_part in text.iter() {
            let forward_count = find_char_ammount_in_str(text_part, &words);
            total += forward_count;
        }

        format!("{total}")
    }

    fn part3(&self) -> String {
        format!("")
    }
}

fn find_char_ammount_in_str(str: &String, words: &Vec<String>) -> usize {
    let mut matches = HashSet::<usize>::with_capacity(str.len());
    for word in words.iter() {
        let mut start = 0;

        while let Some(pos) = str[start..].find(word) {
            let absolute_pos = start + pos;

            for i in 0..word.len() {
                matches.insert(absolute_pos + i);
            }

            start = absolute_pos + 1;
        }
    }
    matches.len()
}
