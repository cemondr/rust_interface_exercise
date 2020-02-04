#[derive(Debug, Default, Clone)]
pub struct KWIndex<'a>(Vec<&'a str>);

// Implementation of the Interface. All method description are in the given specification
impl<'a> KWIndex<'a> {
    pub fn new() -> Self {
        let list: Vec<&'a str> = Vec::new();
        Self(list)
    }
    pub fn extend_from_text(mut self, target: &'a str) -> Self {
        let iterator = target.split_whitespace();
        for mut val in iterator {
            if is_considered_a_word(&val) {
                val = trim_words(val);
                if !val.is_empty() {
                    self.0.push(val);
                }
            }
        }
        Self(self.0)
    }
    pub fn count_matches(&self, keyword: &str) -> usize {
        let mut count: usize = 0;
        for word in &self.0 {
            if *word == keyword {
                count += 1;
            }
        }
        count
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    // My little function for debugging purposes
    pub fn show_all(&self) {
        for word in &self.0 {
            println!("{:?}", word);
        }
    }
}

/*Determines whether a keyword passes the requirements specified (no punctuation in the middle)
if it sees a punctuation somwhere in the middle it makes sure it's not a repetition
for example: "hello..." or "hello!!!!" are valid words
        but: "ain't", "isn't" are NOT valid words
*/
fn is_considered_a_word(keyword: &str) -> bool {
    let len: usize = keyword.len();
    for (count, character) in keyword.chars().enumerate() {
        if !character.is_alphabetic() && is_true_middle(keyword, count, len) {
            return false;
        }
    }
    true
}
/* Helper function to determine where the non alphabetic character actually resides*/
fn is_true_middle(keyword: &str, start: usize, size: usize) -> bool {
    for i in start..size {
        if keyword.chars().nth(i).unwrap().is_alphabetic() {
            return true;
        }
    }
    false
}
/* trims words from numerics and punctuation*/
fn trim_words(mut keyword: &str) -> &str {
    keyword = keyword.trim_matches(char::is_numeric);
    let x: &[_] = &[
        ',', '.', '!', '?', '"', '\'', '*', '(', ')', '#', '@', '^', '&', '$', '{', '}', '[', ']',
        '-', '_', '^', '~',
    ];
    keyword = keyword.trim_matches(x);
    keyword
}
