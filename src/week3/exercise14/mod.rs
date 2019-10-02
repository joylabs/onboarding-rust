pub fn is_match(s: String, p: String) -> bool {
    if p.is_empty() {
        return s.is_empty();
    }

    let first_check = !s.is_empty() && (&p[0..1] == &s[0..1] || &p[0..1] == ".");

    if p.len() >= 2 && &p[1..2] == "*" {
        is_match(s.clone(), (&p[2..]).to_string()) || (first_check && is_match((&s[1..]).to_string(), p.clone()))
    } else {
        first_check && is_match((&s[1..]).to_string(), (&p[1..]).to_string())
    }
}