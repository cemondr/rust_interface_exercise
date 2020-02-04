fn main() {
    let index = kwindex::KWIndex::new().extend_from_text("Hello ain't World.");
    println!("{:?}", &index.len());
    println!("{:?}", &index.count_matches("World"))
}
