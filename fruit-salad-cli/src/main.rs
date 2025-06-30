use clap::Parser;
use fruit_salad_cli::{create_fruit_salad, create_fruit_salad_with_user_fruits};

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "Number of fruits to include in the salad"
)]
struct Opts {
    #[clap(short, long)]
    number: usize,
    #[clap(short, long, num_args = 1..)]
    mine: Option<Vec<String>>,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Get the number of fruits the user requested
    let num_fruits = opts.number;

    match opts.mine {
        Some(mut user_fruits) => {
            let mut salad = create_fruit_salad_with_user_fruits(num_fruits, &mut user_fruits);
            salad.sort();
            println!(
                "Created Fruit salad with {} fruits: {:?}",
                salad.len(),
                salad
            );
        }
        None => {
            let mut salad = create_fruit_salad(num_fruits);

            salad.sort();

            println!(
                "Created fruit salad with {} fruits: {:?}",
                salad.len(),
                salad
            );
        }
    }
}
