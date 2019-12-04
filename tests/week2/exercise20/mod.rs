use onboarding_rust::week2::exercise20::count_of_atoms;

#[test]
fn test_week2_exercise20_test_week2_exercise20_test() {
    let input = "Mg((H2Mg)334)1".to_string();
    let output = "H668Mg335".to_string();
    assert_eq!(output, count_of_atoms(input));
}

#[test]
fn test_week2_exercise20_test_week2_exercise20_k4_mg20_n5_o35_rg15() {
    let input = "K4(ON(Mg2O3)2(Rg)3)5".to_string();
    let output = "K4Mg20N5O35Rg15".to_string();
    assert_eq!(output, count_of_atoms(input));
}

#[test]
fn test_week2_exercise20_test_week2_exercise20_h2_o() {
    let input = "H2O".to_string();
    let output = "H2O".to_string();
    assert_eq!(output, count_of_atoms(input));
}

#[test]
fn test_week2_exercise20_test_week2_exercise20_mg_oh2() {
    let input = "Mg(OH)2".to_string();
    let output = "H2MgO2".to_string();
    assert_eq!(output, count_of_atoms(input));
}

#[test]
fn test_week2_exercise20_test_week2_exercise20_k4_onso322() {
    let input = "K4(ON(SO3)2)2".to_string();
    let output = "K4N2O14S4".to_string();
    assert_eq!(output, count_of_atoms(input));
}

#[test]
fn test_week2_exercise20_test_week2_exercise20_nb333() {
    let input = "(NB3)33".to_string();
    let output = "B99N33".to_string();
    assert_eq!(output, count_of_atoms(input));
}
