use onboarding_rust::week1::exercise6::exercise6;

#[test]
fn test_1_week1_exercise6() {
    let s = String::from("USA");
    assert_eq!(true, exercise6(&s));
}

#[test]
fn test_2_week1_exercise6() {
   let s = String::from("FlaG");
   assert_eq!(false, exercise6(&s));
} 