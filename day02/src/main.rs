use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct PasswordRule {
    letter: char,
    upper: usize,
    lower: usize,
}

impl PasswordRule {
    /// Checks if `word` is a password or not based on counts rule.
    fn check_password_counts(&self, word: &String) -> bool {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for c in word.chars() {
            if counts.contains_key(&c) {
                counts.insert(c, *counts.get(&c).unwrap() + 1);
            } else {
                counts.insert(c, 1);
            }
        }

        let v = match counts.get(&self.letter) {
            Some(v) => *v,
            None => 0,
        };

        v >= self.lower && v <= self.upper
    }

    /// Checks if `word` is a password or not based on places rule.
    fn check_password_places(&self, word: &String) -> bool {
        let is_lower = if &self.lower - 1 < word.len() {
            word.chars().nth(&self.lower - 1).unwrap() == self.letter
        } else {
            false
        };
        let is_upper = if &self.upper - 1 < word.len() {
            word.chars().nth(&self.upper - 1).unwrap() == self.letter
        } else {
            false
        };
        is_lower ^ is_upper
    }
}

fn main() -> std::io::Result<()> {
    let f = File::open("input")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let line_re =
        Regex::new(r"(?P<lower>\d+)-(?P<upper>\d+)\s(?P<letter>\w):\s(?P<word>\w+)").unwrap();
    let mut n_valid_counts = 0;
    let mut n_valid_places = 0;
    loop {
        let len = reader.read_line(&mut line)?;
        if len == 0 {
            break;
        }

        let line_slice = &line[..];
        let caps = line_re.captures(line_slice).unwrap();

        let rule = PasswordRule {
            lower: caps["lower"].parse::<usize>().unwrap(),
            upper: caps["upper"].parse::<usize>().unwrap(),
            letter: caps["letter"].chars().next().unwrap(),
        };
        let word = caps["word"].to_string();

        if rule.check_password_counts(&word) {
            n_valid_counts += 1;
        }
        if rule.check_password_places(&word) {
            n_valid_places += 1;
        }

        line.clear();
    }

    println!("{}", n_valid_counts);
    println!("{}", n_valid_places);

    Ok(())
}
