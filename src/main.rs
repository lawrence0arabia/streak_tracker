use chrono;
use std::io;
use serde::{de::value::Error, Deserialize, Serialize};

fn main() {
    println!("
Hello, world!
This is going to be my new habit tracker.
");

    println!("{}", chrono::offset::Local::now());
    // code to do the things here


}


// Struct to hold the habit data with a list of dates the habit was completed
struct Habit {
    name: String,
    dates: Vec<chrono::DateTime<chrono::offset::Local>>,
}


// function to load habits data from habits.json and return a vector of Habit structs
fn load_habits() -> Vec<Habit> {
    let habits = vec![];
    // code to load habits from habits.json here
    habits
}

// function to create a habit and add it to the habits vector
fn create_habit() -> Option<Habit> {
    let mut habit = Habit {
        name: String::new(),
        dates: vec![],
    };

    io::stdin()
        .read_line(&mut habit.name)
        .expect("Failed to read line");

    // if habit.name is empty, return an error
    if habit.name.is_empty() {
        return None;
    }
    Some(habit)
}

