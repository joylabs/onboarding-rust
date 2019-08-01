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

// #[test]
// fn test_week2_exercise20_example6() {
//     let input = "(((U42Se42Fe10Mc31Rh49Pu49Sb49)49V39Tm50Zr44Og6)33((W2Ga48Tm14Eu46Mt12)23(RuRnMn11)7(Yb15Lu34Ra19CuTb2)47(Md38BhCu48Db15Hf12Ir40)7CdNi21(Db40Zr24Tc27SrBk46Es41DsI37Np9Lu16)46(Zn49Ho19RhClF9Tb30SiCuYb16)15)37(Cr48(Ni31)25(La8Ti17Rn6Ce35)36(Sg42Ts32Ca)37Tl6Nb47Rh32NdGa18Cm10Pt49(Ar37RuSb30Cm32Rf28B39Re7F36In19Zn50)46)38(Rh19Md23No22PoTl35Pd35Hg)41)50".to_string();
//     let expected = "Ar3233800B3408600Bh37Bk78292Ca1900Cd259Ce63000Cl555Cm2815800Cr2400Cu2378Db68635Ds1702Es69782Eu1058F3151395Fe330Ga35304Hf444Hg2050Ho10545I62974In1660600Ir1480La14400Lu28830Mc1023Md48556Mn77Mt276Nb89300Nd1900Ni39527No45100Np15318Og198Pd71750Po2050Pt93100Pu1617Ra893Re611800Rf2447200Rh101922Rn10807Ru87407Sb2623617Se1386Sg79800Si555Sr1702Tb16744Tc45954Ti30600Tl786150Tm1972Ts60800U1386V162987W46Yb9585Zn4397195Zr42300".to_string();
//     assert_eq!(expected, count_of_atoms(input));
// }