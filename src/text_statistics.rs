use std::collections::HashMap;
use rand::random;

pub struct TextStatistics<'a> {
    text_stats: HashMap<&'a str, (HashMap<&'a str, i32>, i32)>,
    max_depth: u32,
}

impl<'a> TextStatistics<'a> {
    pub fn new(text: &'a str, depth: u32) -> TextStatistics<'a> {
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
        
        TextStatistics {
            text_stats: text_stats,
            max_depth: depth,
        }
    }
    
    /// Returns a random word from the source text
    pub fn random_word(&self) -> &str {
        *self.text_stats.keys().nth(random::<usize>() % self.text_stats.len()).unwrap()
    }
    
    /// Returns a word from the source text that could follow the given word.
    /// The chances of any word being selected is proportional to how often
    /// it follows the given word in the source text.
    ///
    /// If the given word never appears in the source text, returns None.
    pub fn random_word_from_word(&'a self, prev_word: &'a str) -> Option<&'a str> {
        if let Some(&(ref word_stats, total)) = self.text_stats.get(prev_word) {
            let mut index = (random::<usize>() % (total as usize)) as i32;
            for (word, count) in word_stats {
                index -= *count;
                if index <= 0 {
                    return Some(word);
                }
            }
            unreachable!();
        }
        else {
            return None;
        }
    }
}

