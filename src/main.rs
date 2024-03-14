use ornamentals::banner_chars::get_banner;
use ornamentals::global::FX_MODE_DEFAULT;

fn main() {
    println!("{}", get_banner("I'm", FX_MODE_DEFAULT));
    println!("{}",get_banner("learning", FX_MODE_DEFAULT));
    println!("{}",get_banner("Rust!", FX_MODE_DEFAULT));
}
