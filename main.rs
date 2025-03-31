use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use std::fs::File;
use std::io;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
struct Card {
    question: String,
    answer: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Subject {
    name: String,
    cards: Vec<Card>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SubjectsWrapper {
    subjects: Vec<Subject>,
}

fn read_json_file(path: &str) -> Result<SubjectsWrapper, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let subjects_data: SubjectsWrapper = from_reader(reader)?;
    Ok(subjects_data)
}

fn get_question<'a>(
    data: &'a SubjectsWrapper,
    subject_name: &str,
    index: usize,
) -> Option<&'a String> {
    data.subjects
        .iter()
        .find(|subject| subject.name == subject_name)
        .and_then(|subject| subject.cards.get(index))
        .map(|card| &card.question)
}

fn main() {
    let data = read_json_file("cards.json").expect("Failed to read JSON");
    let mut input = String::new();
    println!("Enter the subject you want to study: ");
    io::stdin().read_line(&mut input).expect("Invalid input");

    if let Some(question) = get_question(&data, input.trim(), 0) {
        println!("Question: {}", question);
    } else {
        println!("Question not found.");
    }
}
