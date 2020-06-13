use std::collections::HashMap;

fn main() {
    let is_split_char = | chr: char | {
        chr.is_whitespace() || chr.is_ascii_punctuation()
    };

    let s = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    let pos = [1, 5, 6, 7, 8, 9, 15, 16, 19];

    let s: Vec<_> = s.split(is_split_char).filter(|x| x.len() != 0).collect();

    // make map
    let s: HashMap<_,_> = s.iter().enumerate()
        .map(|(i, x)|
            {
                let i = i + 1;
                if pos.contains(&i) {
                    (x.get(0..1).unwrap(), i)
                } else {
                    (x.get(0..2).unwrap(), i)
                }
            })
        .collect();

    // sort
    let mut s: Vec<_> = s.iter().collect();
    s.sort_by(|(_,v1), (_,v2)| v1.cmp(v2));

    for (k,v) in s {
        println!("{}\t{}", k, v)
    }
}
