use std::collections::HashSet;

pub fn isomorphic_string(s: String, t: String) -> bool {
    if s.len() != t.len() {
        println!("lenght");
        return false;
    }
    let s2: HashSet<char> = s.chars().collect();
    let t2: HashSet<char> = t.chars().collect();
    //let union: HashSet<_> = s2.union(&t2).collect();

    if s2.len() != t2.len(){
        return false;
    }
    suma(s) == suma(t)
}


fn suma(chain: String) -> usize {
    let mut s1: char = '0';
    let mut s2 = 0;
    let mut suma = 0;
    for (i, ch) in chain.chars().enumerate() {
        if ch != s1 {
            s1 = ch;
            s2 = i;
        }
        suma += s2;
    }
    suma
}
