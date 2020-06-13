fn main(){
    let s = "stressed";
    let s: String = s.chars().rev().collect();
    println!("{}", s);
}
