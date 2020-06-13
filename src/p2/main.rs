fn main() {
    let s1 = "パトカー";
    let s2 = "タクシー";
    let s: String =  s1.chars()
        .zip(s2.chars())
        .map(|(s1, s2)| { s1.to_string() + &s2.to_string() })
        .collect();
    println!("{}", s);
}
