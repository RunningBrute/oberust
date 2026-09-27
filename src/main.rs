use crate::configuration::Config;

mod configuration {


    pub struct Config {
        input_filename: String,
        output_filename: Option<String>,
    }

    impl Config {
        pub fn new(args: &Vec<String>) -> Result<Self, String> {
            let input = args.get(1).expect("Source file needed");

            let output_opt = args.iter().find(|&x| *x == "-o");

            let mut arguments = Config{
                input_filename: input.clone(),
                output_filename: Some(String::from("default_bin_name")),
            };

            match output_opt {
                Some(_) => arguments.output_filename = args.get(3).cloned(),
                _ => {}
            }

            Ok(arguments)
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let _config = Config::new(&args).unwrap_or_else(|e: String| panic!("{}", e));

    println!("Hello Oberust!")
}
