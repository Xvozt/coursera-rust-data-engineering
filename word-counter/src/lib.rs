use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, prelude::*};

pub fn count_words(path: &str) -> std::io::Result<HashMap<String, u32>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut frequency_map = HashMap::new();

    for line in reader.lines() {
        let line = line?;

        for word in line.split_whitespace() {
            let word = word
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase();
            if !word.is_empty() {
                *frequency_map.entry(word).or_insert(0) += 1;
            }
        }
    }
    Ok(frequency_map)
}
