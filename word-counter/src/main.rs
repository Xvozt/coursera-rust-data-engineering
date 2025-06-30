use word_counter::count_words;
fn main() {
    
    if let Ok(count) = count_words("example.txt") {
        // for (word, count) in &count {
        //     println!("{word}: {count}");
        let mut word_counts: Vec<(String, u32)> = count.into_iter().collect();
        word_counts.sort_by(|a, b| b.1.cmp(&a.1));
        
        for (word, count) in &word_counts {
            println!("{word}: {count}");
        }

        } else {
        println!("Error happened!")
    }

    
}
