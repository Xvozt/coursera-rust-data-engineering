/*
As with the VecDeque example, this code starts by creating a LinkedList of fruits,
converts it to a Vec for shuffling, and then converts it back to a LinkedList.
After the shuffling, it adds "Pomegranate", "Fig", and "Cherry" to the end of the list.
Finally, it prints out the final fruit salad.

This example shows how to use a LinkedList, but remember that LinkedList
has a higher memory overhead and worse cache locality than Vec or VecDeque,
so it's typically not the best choice unless you have a specific need for the properties
of a linked list. In Rust, it's usually better to use a Vec or VecDeque.

A LinkedList is a doubly-linked list, which means that each element in the list
has a pointer to the next element and the previous element.
A great example of when to use a LinkedList is when you need to insert or remove elements
from the middle of the list.
*/

use clap::Parser;
use rand::rng;
use rand::seq::{IndexedRandom, SliceRandom}; // rand is a random number generation library in Rust
use std::collections::LinkedList;

#[derive(Debug, Parser)]
struct Cli {
    /// Add a fruit at a specific position, format: "position:fruit_name"
    /// Example: --add "2:Mango" adds Mango at position 2
    #[clap(short, long)]
    add: Option<String>,
    #[clap(long)]
    random: bool,
    #[clap(long)]
    remove_pos: Option<usize>,
}

fn main() {
    let cli = Cli::parse();

    let mut fruit: LinkedList<String> = LinkedList::new();
    fruit.push_back("Arbutus".to_string());
    fruit.push_back("Loquat".to_string());
    fruit.push_back("Strawberry Tree Berry".to_string());

    /*
    Please note that converting a LinkedList to a Vec and back to a LinkedList
    isn't a common operation in practice. I included
    it in this example to keep the code as similar as possible
    to the original VecDeque example.
     */

    // Scramble (shuffle) the fruit
    let mut rng = rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    if cli.random {
        if let Some(randomized_fruit) = fruit.choose(&mut rng) {
            println!("You selected random fruit: {randomized_fruit}");
        }
    }

    // Convert it back to LinkedList
    let mut fruit: LinkedList<_> = fruit.into_iter().collect();

    // Add fruits to the both ends of the list after shuffling
    fruit.push_front("Pomegranate".to_string());
    fruit.push_back("Fig".to_string());
    fruit.push_back("Cherry".to_string());

    if let Some(add_command) = cli.add {
        if let Some((pos, name)) = add_command.split_once(":") {
            if let Ok(position) = pos.parse::<usize>() {
                if position <= fruit.len() {
                    println!("Adding {name} at position {position}");
                    let mut tail = LinkedList::new();

                    for _ in 0..fruit.len() - position {
                        if let Some(item) = fruit.pop_back() {
                            tail.push_front(item);
                        }
                    }

                    fruit.push_back(name.to_string());
                    fruit.append(&mut tail);
                } else {
                    println!(
                        "Error: Position {} is out of range. The list has {} items",
                        position,
                        fruit.len()
                    );
                }
            } else {
                println!("Invalid position format. Use number for position");
            }
        } else {
            println!("Error: invalid format. Use 'position:fruit_name', e.g. '2:Mango'");
        }
    }

    if let Some(to_delete_pos) = cli.remove_pos {
        if to_delete_pos < fruit.len() {
            let mut tail: LinkedList<String> = LinkedList::new();
            for _ in 0..fruit.len() - to_delete_pos - 1 {
                if let Some(item) = fruit.pop_back() {
                    tail.push_front(item);
                }
            }

            if let Some(removed) = fruit.pop_back() {
                println!("Removed fruit at position {to_delete_pos}: {removed}");
            }

            fruit.append(&mut tail);
            print_fruit_salad(fruit);
        } else {
            println!(
                "Error: Position {} is out of range. The list has {} items",
                to_delete_pos,
                fruit.len()
            )
        }
    } else {
        print_fruit_salad(fruit);
    }
    // Print out the fruit salad
}

fn print_fruit_salad(salad: LinkedList<String>) {
    println!("Fruit Salad:");
    for (i, item) in salad.iter().enumerate() {
        if i != salad.len() - 1 {
            print!("{item}, ");
        } else {
            println!("{item}");
        }
    }
}
