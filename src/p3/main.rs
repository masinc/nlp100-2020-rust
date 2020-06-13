fn main() {
    let is_split_chars = | chr: char | {
        chr.is_whitespace() || chr.is_ascii_punctuation()
    };
    let s = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
    let s : Vec<_> = s.split(is_split_chars).collect();
    let s : String = s.iter()
        .filter(|x| { x.len() != 0})
        .map(|x| { x.len().to_string() })
        .collect();
    println!("{}", s);
}
