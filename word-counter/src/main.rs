use word_counter::count_words;
fn main() {
    
    if let Ok(count) = count_words("example.txt") {
        for (word, count) in &count {
            println!("{word}: {count}");
        }
    } else {
        println!("Error happened!")
    }

    
}
