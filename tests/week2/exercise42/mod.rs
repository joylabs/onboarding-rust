use onboarding_rust::week2::exercise42::count_of_atoms;

#[test]
fn one_test_count_of_atoms() {
    let formula = "H2O".to_string();
    let output = "H2O".to_string();
    assert_eq!(output, count_of_atoms(formula));
}
#[test]
fn two_test_count_of_atoms() {
    let formula = "(Mg(OH)2)2H(TS)2".to_string();
    let output = "H5Mg2O4S2T2".to_string();
    assert_eq!(output, count_of_atoms(formula));
}

#[test]
fn three_test_count_of_atoms() {
    let formula = "K4(ON(SO3)2)2".to_string();
    let output = "K4N2O14S4".to_string();
    assert_eq!(output, count_of_atoms(formula));
}


#[test]
fn four_test_count_of_atoms() {
    let formula =
        "((N42)24(OB40Li30CHe3O48LiNN26)33(C12Li48N30H13HBe31)21(BHN30Li26BCBe47N40)15(H5)16)14"
            .to_string();
    let output = "B18900Be18984C4200H5446He1386Li33894N50106O22638".to_string();
    assert_eq!(output, count_of_atoms(formula));
}

#[test]
fn five_test_count_of_atoms() {
    let formula = "(B2O39He17BeBe49)39".to_string();
    let output = "B78Be1950He663O1521".to_string();
    assert_eq!(output, count_of_atoms(formula));
}