extern crate rand;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use rand::random;

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
    let mut text_stats: HashMap<&str, (HashMap<&str, i32>, i32)> = HashMap::new();
    let mut prev_word = "";
    for word in text.split_whitespace() {
        if text_stats.contains_key(prev_word) {
            let &mut(ref mut word_stats, ref mut total) = text_stats.get_mut(prev_word).unwrap();
            *total += 1;
            if word_stats.contains_key(word) {
                *word_stats.get_mut(word).unwrap() += 1;
            }
            else {
                word_stats.insert(word, 1);
            }
        }
        else {
            text_stats.insert(prev_word, {
                let mut word_stats = HashMap::new();
                word_stats.insert(word, 1);
                (word_stats, 1)
            });
        }
        prev_word = word;
    }
    
    // Print hilarious text
    let mut prev_word = *text_stats.keys().nth(random::<usize>() % text_stats.len()).unwrap();
    print!("{} ", prev_word);
    for _ in 0..50 {
        if let Some(&(ref word_stats, total)) = text_stats.get(prev_word) {
            let mut index = (random::<usize>() % (total as usize)) as i32;
            for (word, count) in word_stats {
                index -= *count;
                if index <= 0 {
                    print!("{} ", word);
                    prev_word = word;
                    break;
                }
            }
        }
        else {
            prev_word = *text_stats.keys().nth(random::<usize>() % text_stats.len()).unwrap();
            print!("{} ", prev_word);
        }
    }
    
    print!("\n\n");
}
