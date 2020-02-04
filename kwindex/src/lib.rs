#[derive(Debug, Default, Clone)]
pub struct KWIndex<'a>(Vec<&'a str>);

/*
trait Trait {
    fn new() -> Self;
    fn extend_from_text<'a>(mut self, target: &'a str) -> Self;
    fn count_matches(&self, keyword: &str) -> usize;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}
*/



impl<'a> KWIndex<'a>{
    pub fn new() -> Self{
        let list: Vec<&'a str> = Vec::new();
        Self(list)
    }
    pub fn extend_from_text(mut self, target: &'a str)-> Self{
        let iterator = target.split_whitespace();
        for mut val in iterator{
            if is_considered_a_word(&val){
                val=trim_words(val);
                self.0.push(val);
            }
        }
        Self(self.0)
    }
    pub fn count_matches(&self, keyword: &str) ->usize{
        let mut count:usize = 0;
        for word in &self.0{
            if *word == keyword{
                count = count +1;
            }
        }
        count
    }
    pub fn len(&self) -> usize{
        self.0.len()
    }
    pub fn is_empty(&self) -> bool{
        self.0.is_empty()
    }
}

fn is_considered_a_word(keyword: &str) -> bool{
    let len = keyword.len();
    let mut count = 0;
    for character in keyword.chars(){
        if !character.is_alphabetic() && count < len-1 && count > 0{
            return false;
        }
        count = count +1;
    } 
    true
}

fn trim_words(mut keyword: &str) -> &str {
    keyword = keyword.trim_matches(char::is_numeric);
    let x: &[_] = &[',','.','!','?','"','\'','*','(',')','#','@','^','&','$','{','}','[',']',
    '-','_','^','~'];
    keyword = keyword.trim_matches(x);
    keyword
}