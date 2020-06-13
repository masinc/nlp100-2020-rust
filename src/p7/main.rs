use std::fmt::Display;

fn main() {
    println!("{}", tpl(12, "気温", 22.4))
}

fn tpl<TX: Display, TY: Display, TZ: Display>(x: TX, y: TY, z: TZ) -> String {
    format!("{}時の{}は{}", x, y, z)
}
