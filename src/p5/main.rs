use std::convert::From;

fn main() {
    let s = String::from("I am an NLPer");
    println!("{:?}", n_gram_str(s.chars(), 1));
    println!("{:?}", n_gram_str(s.chars(), 2));
    println!("{:?}", n_gram_str(s.chars(), 3));

    let x: Vec<_> = s.split_ascii_whitespace().map(String::from).collect();
    println!("{:?}", n_gram(x.iter(), 1));
    println!("{:?}", n_gram(x.iter(), 2));
    println!("{:?}", n_gram(x.iter(), 3));
}

fn n_gram_str<TX: Iterator<Item=char>>(x: TX, n: usize) -> Vec<String>
{
    n_gram(x, n).iter()
        .map(|xs|
            xs.iter().map(|xss| xss.clone()).collect::<String>()
        )
    .collect()
}

fn n_gram<TX, TXS>(x: TX, n: usize) -> Vec<Vec<TXS>>
    where
        TX: Iterator<Item=TXS>,
        TXS: Clone
{
    let vx: Vec<_> = x.collect();
    let len = vx.len();

    (0..len).filter_map(|offset| {
        if (len - offset) >= n {
            let nx = vx.iter().skip(offset).take(n).map(TXS::clone);
            Option::Some(nx.collect())
        } else {
            Option::None
        }
    }).collect()
}
