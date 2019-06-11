use onboarding_rust::week1::exercise12::palindrome;

#[test]
fn test_week1_exercise12_example1() {

    let input = String::from("A man, a plan, a canal: Panama");
    let expected = true;

    assert_eq!(expected, palindrome(input));
}

#[test]
fn test_week1_exercise12_example2() {

    let input = "race a car".to_string();
    let expected = false;

    assert_eq!(expected, palindrome(input));
}

#[test]
fn test_week1_exercise12_example3() {

    let input = "üAdivina ya te opina, ya ni miles origina, ya ni cetro me domina, 
    ya ni monarcas, a repaso ni mulato carreta, acaso nicotina, ya ni cita vecino, 
    anima cocina, pedazo gallina, cedazo terso nos retoza de canilla goza, de panico 
    camina, ónice vaticina, ya ni tocino saca, a terracota luminosa pera, sacra nómina 
    y animo de mortecina, ya ni giros elimina, ya ni poeta, ya ni vidaü"
        .to_string();
    let expected = true;

    assert_eq!(expected, palindrome(input));
}