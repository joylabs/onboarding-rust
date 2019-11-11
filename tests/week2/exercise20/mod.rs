use onboarding_rust::week2::exercise20::count_of_atoms;

#[test]
fn test_week2_exercise20_no_parentheses() {
    let input = "Mg2K4ONSO3".to_string();
    let output = "K4Mg2NO4S".to_string();
    assert_eq!(output, count_of_atoms(input));
}

#[test]
fn test_week2_exercise20_no_num() {
    let input = "MgKNSO".to_string();
    let output = "KMgNOS".to_string();
    assert_eq!(output, count_of_atoms(input));
}

#[test]
fn test_week2_exercise20_parentheses() {
    let input = "Mg(OH)2".to_string();
    let output = "H2MgO2".to_string();
    assert_eq!(output, count_of_atoms(input));
}

// #[test]
// fn test_week2_exercise20_H2O() {
//     let input = "H2O".to_string();
//     let output = "H2O".to_string();
//     assert_eq!(output, count_of_atoms(input));
// }

// #[test]
// fn test_week2_exercise20_MgOH2() {
//     let input = "Mg(OH)2".to_string();
//     let output = "H2MgO2".to_string();
//     assert_eq!(output, count_of_atoms(input));
// }

// #[test]
// fn test_week2_exercise20_K4ONSO322() {
//     let input = "K4(ON(SO3)2)2".to_string();
//     let output = "K4N2O14S4".to_string();
//     assert_eq!(output, count_of_atoms(input));
// }
