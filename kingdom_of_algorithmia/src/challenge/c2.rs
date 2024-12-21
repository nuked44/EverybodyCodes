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
        // Wrong output 
        // Words: THE, HER
        // expected e.g. THERE => 4 THERe
        // actual        THERE => 6 THE_HERe
        // let text = &self.p2_text;
        // let words = self.p2_words.clone();
        // let mut total = 0;
        // for text_part in text.iter() {
        //     let rev_text = text_part.chars().rev().collect::<String>();
        //     for word in words.iter() {
        //         let forward_count = text_part.matches(word).count() * word.len();
        //         let backward_count = rev_text.matches(word).count() * word.len();
        //         total += forward_count;
        //         total += backward_count;
        //     }
        // }
        
        format!("")
    }

    fn part3(&self) -> String {
        format!("")
    }
}
