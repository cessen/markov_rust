extern crate rand;

mod text_statistics;
use text_statistics::TextStatistics;

use std::fs::File;
use std::io::Read;

fn main() {
    let order = 5;

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
    let text_stats = TextStatistics::new_from_text(&text, order);
    
    // Create hilarious text
    let target_word_count = 150;
    let mut words = Vec::new();
    while words.len() < target_word_count {
        let lerp_order = {
            let temp = (target_word_count as u32 - words.len() as u32) / (target_word_count as u32 / (order+1));
            if order < temp {order} else {temp}
        };
        let begin_i = {
            let temp = (words.len() as i32) - (lerp_order as i32);
            if 0 > temp { 0 } else { temp as usize }
        };
        let end_i = words.len();
        
        let word = text_stats.random_word_from_word_list_graceful(&words[begin_i..end_i]);
        words.push(word);
    }
    
    // Print hilarious text
    print!("\n");
    for word in words {
        print!("{} ", word);
    }
    print!("\n\n");
}


