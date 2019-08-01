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

#[test]
fn test_week2_exercise20_example4() {
    let input = "(H12O2)".to_string();
    let expected = "H12O2".to_string();
    assert_eq!(expected, count_of_atoms(input));
}

#[test]
fn test_week2_exercise20_example5() {
    let input = "(H2O2)3".to_string();
    let expected = "H6O6".to_string();
    assert_eq!(expected, count_of_atoms(input));
}

#[test]
fn test_week2_exercise20_example6() {
    let input = "((N7Li31C7B10Be37B23H2H11Li40Be15)26(OBLi48B46N4)25(O48C22He)2N10O34N15B33Li39H34H26B15B23C31(C36N38O33Li38H15H46He21Be38B50)8)3".to_string();
    let expected = "B7512Be4968C1635H2658He510Li10167N1833O1257".to_string();
    assert_eq!(expected, count_of_atoms(input));
}

#[test]
fn test_week2_exercise20_example7() {
    let input = "(N13O9Be)37(LiC50B35)38(Li33HHBe14He5ON50N)27(H3C)2He14C34Li33C33He15N14N5Li24Li17H28O13H42(HeHe6CO11Li)35(He3O27HO5N21H49O39CH37B3)8(O41He27He46He22He17)12".to_string();
    let expected = "B1354Be415C2012H826He1777Li1038N2045O1818".to_string();
    assert_eq!(expected, count_of_atoms(input));
}

#[test]
fn test_week2_exercise20_example8() {
    let input = "(Fe10Ds47)34Nh13Bi21Cd7Tc29Ru45Ne33(Ts9He15Cs38Md30Ne23Gd5)15HAl27Eu36(Ta33Mn25Hg20Ir42Er50Cd5Pa35K)43".to_string();
    let expected = "Al27Bi21Cd222Cs570Ds1598Er2150Eu36Fe340Gd75HHe225Hg860Ir1806K43Md450Mn1075Ne378Nh13Pa1505Ru45Ta1419Tc29Ts135".to_string();
    assert_eq!(expected, count_of_atoms(input));
}

#[test]
fn test_week2_exercise20_example9() {
    let input = "(((U42Se42Fe10Mc31Rh49Pu49Sb49)49V39Tm50Zr44Og6)33((W2Ga48Tm14Eu46Mt12)23(RuRnMn11)7(Yb15Lu34Ra19CuTb2)47(Md38BhCu48Db15Hf12Ir40)7CdNi21(Db40Zr24Tc27SrBk46Es41DsI37Np9Lu16)46(Zn49Ho19RhClF9Tb30SiCuYb16)15)37(Cr48(Ni31)25(La8Ti17Rn6Ce35)36(Sg42Ts32Ca)37Tl6Nb47Rh32NdGa18Cm10Pt49(Ar37RuSb30Cm32Rf28B39Re7F36In19Zn50)46)38(Rh19Md23No22PoTl35Pd35Hg)41)50".to_string();
    let expected = "Ar3233800B3408600Bh12950Bk3914600Ca70300Cd1850Ce2394000Cl27750Cm2815800Cr91200Cu736300Db3598250Ds85100Es3489100Eu1957300F3396150Fe808500Ga2076600Hf155400Hg2050Ho527250I3148700In1660600Ir518000La547200Lu4317900Mc2506350Md539250Mn142450Mt510600Nb89300Nd1900Ni1511350No45100Np765900Og9900Pd71750Po2050Pt93100Pu3961650Ra1652050Re611800Rf2447200Rh4089150Rn423350Ru100350Sb6583650Se3395700Sg2952600Si27750Sr85100Tb1006400Tc2297700Ti1162800Tl83150Tm678200Ts2249600U3395700V64350W85100Yb1748250Zn5729750Zr2115000".to_string();
    assert_eq!(expected, count_of_atoms(input));
}