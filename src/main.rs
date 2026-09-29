use crate::configuration::Config;

mod configuration;

fn main() {
    let mut args = std::env::args();
    let _config = Config::new(&mut args).unwrap_or_else(|e: String| panic!("{}", e));

    println!("Hello Oberust!")
}
