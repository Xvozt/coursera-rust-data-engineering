/*
This code starts with an initial VecDeque,
converts it to a Vec for shuffling, and then converts it back to a VecDeque.
After that, it pushes "Pomegranate" to the front of the deque, and "Fig" and "Cherry"
to the back of the deque. Finally, it prints out the final fruit salad.

A VecDeque is a double-ended queue, which means that you can push and pop from both ends
of the queue.
*/

use clap::Parser;
use rand::rng;
use rand::seq::{IndexedRandom, SliceRandom}; // rand is a random number generation library in Rust
use std::collections::VecDeque;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(short, long)]
    front: Option<String>,
    #[clap(short, long)]
    back: Option<String>,
    #[clap(short, long)]
    choose: bool,
    #[clap(short, long)]
    remove: bool,
}
fn main() {
    let cli = Cli::parse();

    let mut fruit: VecDeque<String> = VecDeque::new();
    fruit.push_back("Arbutus".to_string());
    fruit.push_back("Loquat".to_string());
    fruit.push_back("Strawberry Tree Berry".to_string());

    // Scramble (shuffle) the fruit
    let mut rng = rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to VecDeque
    let mut fruit: VecDeque<_> = fruit.into_iter().collect();

    // Add fruits to the both ends of the queue after shuffling
    fruit.push_front("Pomegranate".to_string());
    fruit.push_back("Fig".to_string());
    fruit.push_back("Cherry".to_string());

    if let Some(back_fruit) = cli.back {
        println!("Adding {back_fruit} to the back of the fruit salad");
        fruit.push_back(back_fruit);
    }

    if let Some(front_fruit) = cli.front {
        println!("Adding {front_fruit} to the front of the fruit salad");
        fruit.push_front(front_fruit);
    }

    // if cli.choose {
    //     if let Some(random_fruit) = fruit.iter().collect::<Vec<_>>().choose(&mut rng) {
    //         println!("Randomly chosen fruit: {}", random_fruit);
    //     } else {
    //         println!("No fruits available to choose from");
    //     }
    // }

    if cli.choose {
        let fruit_slice = fruit.make_contiguous();
        if let Some(random_fruit) = fruit_slice.choose(&mut rng) {
            println!("Randomly chosen fruit is {random_fruit}");
        } else {
            println!("No fruits available to choose from");
        }
    }

    if cli.remove {
        if let Some(removed_fruit) = fruit.pop_front() {
            println!("Removed fruit is: {removed_fruit}");
        } else {
            println!("Nothing to remove");
        }
    }
    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{item}, ");
        } else {
            println!("{item}");
        }
    }
}
