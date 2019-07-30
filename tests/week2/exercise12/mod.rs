use onboarding_rust::week2::exercise12::MyHashSet;

#[test]
fn test_week2_exercise12_example1() {
    let mut hash_set: MyHashSet = MyHashSet::new();
    hash_set.add(1);
    hash_set.add(2);
    assert_eq!(true, hash_set.contains(1));
    assert_eq!(false, hash_set.contains(3));

    hash_set.add(2);
    assert_eq!(true, hash_set.contains(2));

    hash_set.remove(2);
    assert_eq!(false, hash_set.contains(2));
}