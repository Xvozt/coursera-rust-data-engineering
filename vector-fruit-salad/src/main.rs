/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array. It can grow or shrink in size and is one of the most
useful data structures in Rust. A vector is represented using the Vec<T> type.
*/

use rand::seq::{IndexedRandom, SliceRandom}; // rand is a random number generation library in Rust
use rand::rng;
use clap::Parser;
use std::cmp::min;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(short, long)]
    fruit: Option<String>,
    #[clap(short, long)]
    number: Option<usize>,
    #[clap(short, long)]
    random: Option<usize>,
}

fn main() {

    let cli = Cli::parse();
    
    let mut fruits = vec![
        "Orange".to_string(),
        "Fig".to_string(),
        "Pomegranate".to_string(),
        "Cherry".to_string(),
        "Apple".to_string(),
        "Pear".to_string(),
        "Peach".to_string(),
    ];

    // Additional fruits that can be randomly added
    let additional_fruits = [
        "Banana".to_string(),
        "Strawberry".to_string(),
        "Blueberry".to_string(),
        "Mango".to_string(),
        "Kiwi".to_string(),
        "Pineapple".to_string(),
        "Watermelon".to_string(),
        "Grape".to_string(),
    ];
    
    // add user fruit
    if let Some(user_fruit) = cli.fruit {
        println!("Adding {user_fruit} to the fruit salad");
        fruits.push(user_fruit);
    }
    
    // add user defined random number of aditional fruits
    if let Some(random_num) = cli.random {
        let mut rng = rng();
        println!("Adding {random_num} random fruits to the salad");

        for _ in 0..random_num {
            if let Some(random_fruit) = additional_fruits.choose(&mut rng).cloned() {
                println!("Added random fruit: {random_fruit}");
                fruits.push(random_fruit);
            }
        }
    }

  

    // Scramble (shuffle) the fruit 
    let mut rng = rng();
    fruits.shuffle(&mut rng);

    if let Some(user_number) = cli.number {
        println!("Taking only {user_number} fruits to the salad");
        let actual_number = min(user_number, fruits.len());

        println!("Fruit Salad:");

        for (i, item) in fruits.iter().enumerate().take(actual_number) {
            if i != actual_number - 1 {
                print!("{item}, ");
            } else {
                print!("{item}");
            }
        }
    } else {
        // printing all fruits if limit is not specified
        println!("Fruit salad:");
        for (i, item) in fruits.iter().enumerate() {
            if i != fruits.len() - 1 {
                print!("{item}, ");
            } else {
                print!("{item}");
            }
        }
    } 
    
}