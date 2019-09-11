use onboarding_rust::week2::exercise22::Trie;

#[test]
fn test_week2_exercise22_example1() {
    let mut trie: Trie = Trie::new();

    trie.insert("apple".to_string());
    assert!(trie.search("apple".to_string()));

    assert!(!trie.search("app".to_string()));
    assert!(trie.starts_with("app".to_string()));

    trie.insert("app".to_string());
    assert!(trie.search("app".to_string()));
}

#[test]
fn test_week2_exercise22_example2() {
    let trie: Trie = Trie::new();

    //Looking in an empty trie
    assert!(!trie.search("none".to_string()));
    assert!(!trie.starts_with("non".to_string()));
}

#[test]
fn test_week2_exercise22_example3() {
    let mut trie: Trie = Trie::new();

    trie.insert("apple".to_string());

    //searching/starts_with using longer string inserted
    assert!(!trie.starts_with("applebe".to_string()));
    assert!(!trie.search("applebees".to_string()));
}