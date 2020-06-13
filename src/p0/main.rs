use std::path::{PathBuf};
use std::io::{self, Result};

fn pwd() -> io::Result<PathBuf> {
    let pwd = std::env::current_dir()?;
    io::Result::Ok(pwd)
}

fn pwd_str<'a>() -> Result<String> {
    let pwd = pwd()?;
    match pwd.to_str() {
        Some(s) => Result::Ok(String::from(s)),
        None => Result::Err(io::Error::from(io::ErrorKind::Other))
    }
}

fn main() -> Result<()> {
    let s = "stressed";
    let s: String = s.chars().rev().collect();
    println!("{}", s);

    let pwd = pwd_str()?;
    println!("{}", pwd);

    Result::Ok(())
}
//=("abc")
//(= + "Hello" "" "Wold")
//( echo "Hello World" )
/*(=
)*/
