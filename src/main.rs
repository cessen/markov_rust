extern crate rand;
extern crate regex;

mod stats;

use std::fs::File;
use std::io::Read;
use std::collections::VecDeque;

use regex::Regex;

use stats::MarkovStats;

fn main() {
    // Read input text to a string
    let text = {
        let mut text = String::new();
        let _ = File::open("text.txt").unwrap().read_to_string(&mut text);
        Regex::new(r"(?P<a>[^\n])\n(?P<b>[^\n])").unwrap().replace_all(&text, "$a $b")
    };

    // Generate statistics
    const MAX_ORD: usize = 6;
    let stats = MarkovStats::from_str(&text, MAX_ORD);

    // Generate hilarious text
    let mut gen_text = String::new();
    let mut window = VecDeque::new();
    window.push_back(0);
    for _ in 0..500 {
        if let Some(c) = stats.markov_char(&gen_text[window[0]..]) {
            gen_text.push(c);
        } else {
            gen_text.push(stats.random_char());
            while window.len() > 1 {
                window.pop_front();
            }
        }

        window.push_back(gen_text.len());
        if window.len() > (MAX_ORD + 1) {
            window.pop_front();
        }
    }

    println!("{}", gen_text);
}
