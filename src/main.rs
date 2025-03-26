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

    let mut habits = Vec::new();
    let mut habit: Option<Habit> = create_habit();
    if let Some(habit) = habit {
        habits.push(habit);
    }

    habit = create_habit();
    if let Some(habit) = habit {
        habits.push(habit)
    }

    list_habits(habits)
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

    println!("New habit name: ");

    io::stdin()
        .read_line(&mut habit.name)
        .expect("Failed to read line");

    // if habit.name is empty, return none
    if habit.name.is_empty() {
        return None;
    }

    habit.name = habit.name.trim().to_string();

    println!("New habit \"{}\" created!", habit.name);

    Some(habit)
}

fn list_habits(habits: Vec<Habit>) {
    for habit in habits.iter() {
        println!("Habit: {}", habit.name)
    }
}