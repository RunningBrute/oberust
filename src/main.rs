use crate::configuration::Config;

mod configuration;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let _config = Config::new(&args).unwrap_or_else(|e: String| panic!("{}", e));

    println!("Hello Oberust!")
}
