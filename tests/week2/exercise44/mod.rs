use onboarding_rust::week2::exercise44::Trie;

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
