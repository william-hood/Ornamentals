mod banner_chars;
use crate::banner_chars::get_banner;


fn main() {
    println!("{}", get_banner("I'm", 0));
    println!("{}",get_banner("learning", 0));
    println!("{}",get_banner("Rust!", 0));
}
