use onboarding_rust::week2::exercise15::subdomain_visits;

#[test]
fn test_week2_exercise15_example1() {
    let input = vec!["9001 discuss.leetcode.com"];
    let output = vec!["9001 discuss.leetcode.com", "9001 leetcode.com", "9001 com"];
    assert_eq!(output, subdomain_visits(input));
}

#[test]
fn test_week2_exercise15_example2() {
    let input = vec!["900 google.mail.com", "50 yahoo.com", "1 intel.mail.com", "5 wiki.org"];
    let output = vec!["901 mail.com","50 yahoo.com","900 google.mail.com","5 wiki.org","5 org","1 intel.mail.com","951 com"];
    assert_eq!(output, subdomain_visits(input));
}
