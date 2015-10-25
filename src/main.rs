extern crate rand;

mod text_statistics;
use text_statistics::TextStatistics;

use std::fs::File;
use std::io::Read;

fn main() {
    // Read the text in
    let text = if let Ok(mut f) = File::open("text.txt") {
        let mut s = String::new();
        if let Err(_) = f.read_to_string(&mut s) {
            panic!("text.txt does not contain valid UTF-8 text.");
        }
        s
    }
    else {
        panic!("Could not open text.txt");
    };
    
    // Build text statistics
    let text_stats = TextStatistics::new(&text, 3);
    
    // Print hilarious text
    let mut prev_word = text_stats.random_word();
    print!("{} ", prev_word);
    for _ in 0..50 {
        if let Some(word) = text_stats.random_word_from_word(prev_word) {
            prev_word = word;
            print!("{} ", prev_word);
        }
        else {
            prev_word = text_stats.random_word();
            print!("{} ", prev_word);
        }
    }
    
    print!("\n\n");
}


