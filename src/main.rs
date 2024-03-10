use std::cmp::min;

fn main() {
    use std::io::{stdin, stdout, Write};
    let mut s1: String = String::new();
    print!("Please enter string1: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s1)
        .expect("Did not enter a correct string");
    if let Some('\n') = s1.chars().next_back() {
        s1.pop();
    }
    if let Some('\r') = s1.chars().next_back() {
        s1.pop();
    }
    let mut s2: String = String::new();
    print!("Please enter string2: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s2)
        .expect("Did not enter a correct string");
    if let Some('\n') = s2.chars().next_back() {
        s2.pop();
    }
    if let Some('\r') = s2.chars().next_back() {
        s2.pop();
    }

    let ld = levenshtein_distance(s1, s2);
    println!("{}", ld);
}

fn levenshtein_distance(s1: String, s2: String) -> usize {
    // DEFINITION
    // lev(a,b)
    // = |a| if |b| == 0,
    // = |b| if |a| == 0,
    // = lev( tail(a), tail(b) ) if head(a) == head(b),
    // = 1 + min(lev(tail(a), b), lev(a, tail(b)), lev( tail(a), tail(b) )) otherwise
    if s1.len() == 0 {
        return s2.len();
    } else if s2.len() == 0 {
        return s1.len();
    } else if s1[..1] == s2[..1] {
        return levenshtein_distance(s1[1..].to_string(), s2[1..].to_string());
    } else {
        return 1 + min(
            min(
                levenshtein_distance(s1[1..].to_string(), s2.clone()),
                levenshtein_distance(s1.clone(), s2[1..].to_string()),
            ),
            levenshtein_distance(s1[1..].to_string(), s2[1..].to_string()),
        );
    }
}
