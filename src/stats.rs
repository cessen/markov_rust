use std::collections::HashMap;
use std::collections::VecDeque;

use rand::random;

#[derive(Debug)]
pub struct MarkovStats<'a> {
    text: &'a str,
    stats: HashMap<&'a str, HashMap<char, u32>>,
    max_ord: usize,
}

const MAX_ORDER: usize = 1000;

impl<'a> MarkovStats<'a> {
    pub fn from_str(text: &'a str) -> MarkovStats<'a> {
        let mut stats = HashMap::new();
        let mut max_order = 0;
        let mut ord_stats = HashMap::new();
        let mut window = VecDeque::new();
        for ord in 1..MAX_ORDER {
            window.clear();
            for (i, c) in text.char_indices() {
                window.push_back(i);
                if window.len() == (ord + 1) {
                    let s = &text[*window.front().unwrap()..*window.back().unwrap()];
                    if ord == 1 || stats.contains_key(&s[1..]) {
                        *ord_stats.entry(s)
                                  .or_insert_with(|| HashMap::new())
                                  .entry(c)
                                  .or_insert(0) += 1;
                    }
                    window.pop_front();
                }
            }

            // Merge into main stats
            let mut merge_count = 0;
            for (k, v) in ord_stats.iter() {
                if v.len() > 1 {
                    stats.insert(*k, v.clone());
                    merge_count += 1;
                }
            }
            if merge_count == 0 {
                max_order = ord - 1;
                break;
            }
            ord_stats.clear();
        }

        MarkovStats {
            text: text,
            stats: stats,
            max_ord: max_order,
        }
    }

    pub fn max_order(&self) -> usize {
        self.max_ord
    }

    pub fn random_char(&self) -> char {
        let n = random::<usize>() % self.stats.len();
        self.stats.keys().nth(n).unwrap().chars().nth(0).unwrap()
    }

    pub fn markov_char(&self, key: &str) -> Option<char> {
        if key.chars().count() == 0 {
            return None;
        }

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
            return self.find_unique(key);
        }
    }

    fn find_unique(&self, key: &str) -> Option<char> {
        let char_count = key.chars().count();

        let mut window = VecDeque::new();
        for (i, c) in self.text.char_indices() {
            // Move sliding window
            window.push_back(i);
            if window.len() == (char_count + 1) {
                let s = &self.text[*window.front().unwrap()..*window.back().unwrap()];
                if key == s {
                    return Some(c);
                }
                window.pop_front();
            }
        }

        return None;
    }
}
