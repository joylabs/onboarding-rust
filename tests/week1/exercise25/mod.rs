use onboarding_rust::week1::exercise25::num_jewels_in_stones;

#[test]
fn test_1_exercise25() {
    let j = "aAAbbbb".to_string();
    let s = "aA".to_string();
    let jewels = 3;
    assert_eq!(jewels, num_jewels_in_stones(j, s));
}

#[test]
fn test_2_exercise25() {
    let j = "ZZ".to_string();
    let s = "z".to_string();
    let jewels = 0;
    assert_eq!(jewels, num_jewels_in_stones(j, s));
}


#[test]
fn test_3_exercise25() {
    let j = "abshhiOSJENCJesfe".to_string();
    let s = "hOe".to_string();
    let jewels = 5;
    assert_eq!(jewels, num_jewels_in_stones(j, s));
}