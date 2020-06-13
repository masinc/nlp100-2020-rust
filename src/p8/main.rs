use std::char;

fn main() {
    let s1 = cipher("message");
    let s2 = cipher(&s1);
    println!("{}", s1);
    println!("{}", s2);
}

fn cipher(s: &str) -> String {
    s.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let n = 218 - (c as u32);
            char::from_u32(n).unwrap()
        } else {
            c
        }
    }).collect()
}
