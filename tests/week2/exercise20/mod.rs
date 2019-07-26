use onboarding_rust::week2::exercise20::count_of_atoms;

#[test]
fn test_week2_exercise20_example1() {
    let input = "H2O".to_string();
    let expected = "H2O".to_string();
    assert_eq!(expected, count_of_atoms(input));
}

#[test]
fn test_week2_exercise20_example2() {
    let input = "Mg(OH)2".to_string();
    let expected = "H2MgO2".to_string();
    assert_eq!(expected, count_of_atoms(input));
}

#[test]
fn test_week2_exercise20_example3() {
    let input = "K4(ON(SO3)2)2".to_string();
    let expected = "K4N2O14S4".to_string();
    assert_eq!(expected, count_of_atoms(input));
}
