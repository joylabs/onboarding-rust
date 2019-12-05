use onboarding_rust::week1::exercise69::count_of_atoms;

#[test]
#[allow(non_snake_case)]
fn test_exercise69_number_of_atoms_H668Mg335() {
    let input = "Mg((H2Mg)334)1".to_string();
    let output = "H668Mg335".to_string();
    assert_eq!(output, count_of_atoms(input));
}

#[test]
#[allow(non_snake_case)]
fn test_exercise69_number_of_atoms_K4Mg20N5O35Rg15() {
    let input = "K4(ON(Mg2O3)2(Rg)3)5".to_string();
    let output = "K4Mg20N5O35Rg15".to_string();
    assert_eq!(output, count_of_atoms(input));
}

#[test]
#[allow(non_snake_case)]
fn test_exercise69_number_of_atoms_H2O() {
    let input = "H2O".to_string();
    let output = "H2O".to_string();
    assert_eq!(output, count_of_atoms(input));
}

#[test]
#[allow(non_snake_case)]
fn test_exercise69_number_of_atoms_H2MgO2() {
    let input = "Mg(OH)2".to_string();
    let output = "H2MgO2".to_string();
    assert_eq!(output, count_of_atoms(input));
}

#[test]
#[allow(non_snake_case)]
fn test_exercise69_number_of_atoms_K4N2O14S4() {
    let input = "K4(ON(SO3)2)2".to_string();
    let output = "K4N2O14S4".to_string();
    assert_eq!(output, count_of_atoms(input));
}

#[test]
#[allow(non_snake_case)]
fn test_exercise69_number_of_atoms_B99N33() {
    let input = "(NB3)33".to_string();
    let output = "B99N33".to_string();
    assert_eq!(output, count_of_atoms(input));
}