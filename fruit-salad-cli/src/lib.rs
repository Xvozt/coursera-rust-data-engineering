use rand::rng;
use rand::seq::SliceRandom;

pub static DEFAULT_FRUITS: &[&str] = &[
    "Arbutus",
    "Loquat",
    "Strawberry Tree Berry",
    "Pomegranate",
    "Fig",
    "Cherry",
    "Orange",
    "Pear",
    "Peach",
    "Apple",
];

pub fn create_fruit_salad(num_fruits: usize) -> Vec<String> {
    let mut fruits: Vec<String> = DEFAULT_FRUITS.iter().map(|&s| s.to_string()).collect();
    let mut rng = rng();

    fruits.shuffle(&mut rng);

    fruits.into_iter().take(num_fruits).collect()
}

pub fn create_fruit_salad_with_user_fruits(
    num_fruits: usize,
    users_fruits: &mut Vec<String>,
) -> Vec<String> {
    let mut fruits: Vec<String> = DEFAULT_FRUITS.iter().map(|&s| s.to_string()).collect();
    fruits.append(users_fruits);

    let mut rng = rng();
    fruits.shuffle(&mut rng);

    fruits.into_iter().take(num_fruits).collect()
}
