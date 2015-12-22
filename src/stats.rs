use std::collections::HashMap;
use std::cell::Cell;

use rand::random;

#[derive(Debug)]
pub struct MarkovStats<'a> {
    text: &'a str,
    stats: HashMap<&'a str, HashMap<char, u32>>,
    max_ord: usize,
    cache_index: Cell<usize>,
}

const MAX_ORDER: usize = 1000;

impl<'a> MarkovStats<'a> {
    /// Creates markov chain statistics from a string.
    pub fn from_str(text: &'a str) -> MarkovStats<'a> {
        let mut stats = HashMap::new();
        let mut max_order = 0;

        for ord in 0..MAX_ORDER {
            // Build the stats for this order
            let mut ord_stats = HashMap::new();
            for ((i1, _), (i2, c)) in Iterator::zip(text.char_indices(),
                                                    text.char_indices().skip(ord)) {
                let s = &text[i1..i2];
                if ord == 0 || stats.contains_key(&s[1..]) {
                    *ord_stats.entry(s)
                              .or_insert_with(|| HashMap::new())
                              .entry(c)
                              .or_insert(0) += 1;
                }
            }

            // Merge into main stats
            let mut merge_count = 0;
            for (k, v) in ord_stats.into_iter() {
                if v.len() > 1 {
                    stats.insert(k, v);
                    merge_count += 1;
                }
            }

            // If there were no stats to be merged, then there can't
            // be any stats needed from the higher orders either.
            // So stop.
            if merge_count == 0 {
                max_order = ord - 1;
                break;
            }
        }

        MarkovStats {
            text: text,
            stats: stats,
            max_ord: max_order,
            cache_index: Cell::new(0),
        }
    }

    /// Returns the maximum order of the stats where there is still
    /// more than one choice for any given query.
    pub fn max_order(&self) -> usize {
        self.max_ord
    }

    /// Returns a random char from the source text.
    pub fn random_char(&self) -> char {
        self.markov_char("").unwrap()
    }

    /// Returns a char that might follow the given text key.
    /// If the text key doesn't exist in the stats, returns None.
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
        } else if let Some(c) = self.find_unique(key, self.cache_index.get()) {
            return Some(c);
        } else {
            return self.find_unique(key, 0);
        }
    }

    fn find_unique(&self, key: &str, start_byte: usize) -> Option<char> {
        let char_count = key.chars().count();
        let text = &self.text[start_byte..];

        for ((i1, _), (i2, c)) in Iterator::zip(text.char_indices(),
                                                text.char_indices().skip(char_count)) {
            if key == &text[i1..i2] {
                self.cache_index.set(start_byte + i1);
                return Some(c);
            }
        }

        return None;
    }
}
