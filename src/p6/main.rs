use std::collections::BTreeSet;

fn main() {
    let x = "paraparaparadise";
    let y = "paragraph";

    let mut ngram_x = n_gram_str(x, 2);
    ngram_x.sort();
    let mut ngram_y = n_gram_str(y, 2);
    ngram_y.sort();

    let set_x: BTreeSet<_> = ngram_x.iter().map(String::clone).collect();
    let set_y: BTreeSet<_> = ngram_y.iter().map(String::clone).collect();

    println!("x={:?}", set_x);
    println!("y={:?}", set_y);

    let union = set_x.union(&set_y).collect::<BTreeSet<_>>();
    println!("union={:?}",union);
    let intersection = set_x.intersection(&set_y).collect::<BTreeSet<_>>();
    println!("intersection={:?}", intersection);
    let difference = set_x.difference(&set_y).collect::<BTreeSet<_>>();
    println!("difference={:?}", difference);

    let se = &String::from("se");
    let contains = set_x.get(se).and(set_y.get(se)).is_some();
    println!("contains-se={}", contains);
}

fn n_gram_str(x: &str, n: usize) -> Vec<String>
{
    n_gram(x.chars(), n).iter()
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
