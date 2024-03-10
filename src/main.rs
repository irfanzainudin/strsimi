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
    println!("You typed s1: {}", s1);
    println!("You typed s2: {}", s2);
}
