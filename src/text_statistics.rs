use std::collections::HashMap;
use rand::random;



pub struct TextStatistics<'a> {
    stats: HashMap<&'a str, (i32, TextStatistics<'a>)>,
    total: i32,
}

impl<'a> TextStatistics<'a> {
    fn insert_instance(&mut self, words: &[&'a str]) {
        // If there are no words, nothing to do
        if words.len() == 0 {
            return;
        }
        
        self.total += 1;
        
        // Increment or create stats for the first word
        if self.stats.contains_key(words[0]) {
            if let Some(&mut (ref mut count, _)) = self.stats.get_mut(words[0]) {
                *count += 1;
            }
        }
        else {
            self.stats.insert(words[0], (1, TextStatistics::new()));
        }
        
        // If there are more words, recurse
        if words.len() > 1 {
            if let Some(&mut (_, ref mut word_stats)) = self.stats.get_mut(words[0]) {
                word_stats.insert_instance(&words[1..]);
            }
        }
    }
    
    
    fn new() -> TextStatistics<'a> {
        TextStatistics {
            stats: HashMap::new(),
            total: 0,
        }
    }
    
    /// Builds a TextStatistics struct from the given source text
    pub fn new_from_text(text: &'a str, depth: u32) -> TextStatistics<'a> {
        let mut text_stats = TextStatistics::new();
        
        let mut rolling_word_window: Vec<&str> = Vec::new();
        for word in text.split_whitespace() {
            if rolling_word_window.len() < ((depth + 1) as usize) {
                rolling_word_window.push(word);
            }
            else {
                text_stats.insert_instance(&rolling_word_window);
                
                let rwwl = rolling_word_window.len();
                for i in 0..(rwwl-1) {
                    rolling_word_window[i] = rolling_word_window[i+1];
                }
                rolling_word_window[rwwl-1] = word;
            }
        }
        
        text_stats
    }
    
    
    /// Returns a random word, with a probability distribution equal to the
    /// distribution in the source text.
    pub fn random_word(&self) -> &str {
        let mut index = (random::<usize>() % (self.total as usize)) as i32;
        for (word, &(count, _)) in &self.stats {
            index -= count;
            if index <= 0 {
                return word;
            }
        }
        unreachable!();
    }
    
    /// Returns a word from the source text that could follow the given words.
    pub fn random_word_from_word_list(&'a self, words: &[&str]) -> Option<&'a str> {
        if words.len() == 0 {
            return Some(self.random_word());
        }
        
        if let Some(&(_, ref word_stats)) = self.stats.get(words[0]) {
            return word_stats.random_word_from_word_list(&words[1..]);
        }
        else {
            return None;
        }
    }
    
    pub fn random_word_from_word_list_graceful(&'a self, words: &[&str]) -> &'a str {
        for i in 0..(words.len()+1) {
            if let Some(word) = self.random_word_from_word_list(&words[i..words.len()]) {
                return word;
            }
        }
        unreachable!();
    }
}

