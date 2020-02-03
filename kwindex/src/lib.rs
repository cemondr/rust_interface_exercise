#[derive(Debug, Default, Clone)]
pub struct KWIndex<'a>(Vec<&'a str>);

trait Trait {
    fn new() -> Self;
    fn extend_from_text<'a>(mut self, target: &'a str) -> Self;
    fn count_matches(&self, keyword: &str) -> usize;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

impl<'a> Trait for KWIndex<'a>{
    fn new() -> Self{
        let list: Vec<&'a str> = Vec::new();
        Self(list)
    }
}