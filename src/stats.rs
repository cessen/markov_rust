use std::collections::HashMap;
use std::collections::VecDeque;

use rand::random;

#[derive(Debug)]
pub struct MarkovStats<'a> {
    stats: HashMap<&'a str, HashMap<char, u32>>,
}

impl<'a> MarkovStats<'a> {
    pub fn from_str(text: &'a str, max_ord: usize) -> MarkovStats<'a> {
        let mut stats = HashMap::new();
        let mut window = VecDeque::new();

        for (i, c) in text.char_indices() {
            // Move sliding window
            window.push_back(i);
            if window.len() > (max_ord + 1) {
                window.pop_front();
            }

            // For each order, add stats
            for ord in 1..window.len() {
                let s = &text[window[window.len() - 1 - ord]..window[window.len() - 1]];
                *stats.entry(s)
                      .or_insert_with(|| HashMap::new())
                      .entry(c)
                      .or_insert(0) += 1;
            }
        }

        MarkovStats { stats: stats }
    }

    pub fn random_char(&self) -> char {
        let n = random::<usize>() % self.stats.len();
        self.stats.keys().nth(n).unwrap().chars().nth(0).unwrap()
    }

    pub fn markov_char(&self, key: &str) -> Option<char> {
        if let Some(substats) = self.stats.get(key) {
            let n = random::<usize>() % (substats.values().fold(0, |acc, n| acc + *n) as usize);
            let mut i = 0;
            for (c, count) in substats.iter() {
                i += *count as usize;
                if i > n {
                    return Some(*c);
                }
            }
            unreachable!()
        } else {
            return None;
        }
    }
}
