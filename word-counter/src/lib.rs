use std::{collections::HashMap};
use std::fs::File;
use std::io::prelude::*;

pub fn count_words(path: &str) -> std::io::Result<HashMap<String, u32>> {
    let mut file = File::open(path)?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let word_list: Vec<&str> = contents.split(' ').collect();

    let mut frequency_map = HashMap::new();

    for word in word_list {
        *frequency_map.entry(word.to_string()).or_insert(0) += 1;
    };

    Ok(frequency_map)
}