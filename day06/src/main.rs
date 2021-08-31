use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let f = File::open("input")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut all_questions_answered: Vec<usize> = Vec::new();
    let mut common_questions_answered: Vec<usize> = Vec::new();

    while reader.read_line(&mut line)? > 0 {
        let mut all_questions: HashSet<char> = HashSet::new();
        let mut common_questions: HashSet<char> = HashSet::new();
        for c in "abcdefghijklmnopqrstuvwxyz".chars() {
            common_questions.insert(c);
        }
        loop {
            let mut questions: HashSet<char> = HashSet::new();
            for c in line.trim().chars() {
                questions.insert(c);
            }
            all_questions = all_questions
                .union(&questions)
                .cloned()
                .collect::<HashSet<char>>();
            common_questions = common_questions
                .intersection(&questions)
                .cloned()
                .collect::<HashSet<char>>();
            line.clear();
            if reader.read_line(&mut line)? == 0 || line.trim().len() == 0 {
                break;
            }
        }
        all_questions_answered.push(all_questions.len());
        common_questions_answered.push(common_questions.len());
        line.clear();
    }

    println!("{}", all_questions_answered.iter().sum::<usize>());
    println!("{}", common_questions_answered.iter().sum::<usize>());

    Ok(())
}
