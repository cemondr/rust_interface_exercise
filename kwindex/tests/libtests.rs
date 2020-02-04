#[test]
fn test_new(){
    let test_struct = kwindex::KWIndex::new();
    assert_eq!(test_struct.len(),0);
    assert_eq!(test_struct.is_empty(),true);
}

#[test]
fn count_matches(){
    let test_struct = kwindex::KWIndex::new().extend_from_text("never ever ever... ever ever!");
    assert_eq!(test_struct.len(),5);
    assert_eq!(test_struct.count_matches("ever"),4);
}

#[test]
fn test_extend_from(){
    let test_struct = kwindex::KWIndex::new().extend_from_text("Hello World ain't 1 word!!!");
    assert_eq!(test_struct.len(),3);
    assert_eq!(test_struct.count_matches("word"),1);
}

#[test]
fn test_extend_from_chain(){
    let test_struct = kwindex::KWIndex::new()
    .extend_from_text("One")
    .extend_from_text("is the")
    .extend_from_text("loneliest")
    .extend_from_text("number");

    assert_eq!(test_struct.len(),5);
}

