use onboarding_rust::week2::exercise11::MyHashMap;

#[test]
fn test_week2_exercise11_put() {
    let mut hash_map: MyHashMap = MyHashMap::new();
    hash_map.put(1, 1);
    assert_eq!(1, hash_map.get(1));
}

#[test]
fn test_week2_exercise11_empty_get() {
    let hash_map: MyHashMap = MyHashMap::new();
    assert_eq!(-1, hash_map.get(3));
}

#[test]
fn test_week2_exercise11_modified_get() {
    let mut hash_map: MyHashMap = MyHashMap::new();
    hash_map.put(2, 1);
    hash_map.put(2, 5);
    assert_eq!(5, hash_map.get(2));
}

#[test]
fn test_week2_exercise11_remove() {
    let mut hash_map: MyHashMap = MyHashMap::new();
    hash_map.put(2, 1);
    hash_map.remove(2);
    assert_eq!(-1, hash_map.get(2));
}
