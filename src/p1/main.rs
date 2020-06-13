fn main () {
    let s = "パタトクカシーー";
    let s: String = s.chars().step_by(2).collect();
    println!("{}", s);
}
