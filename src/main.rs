use crate::configuration::Config;

mod configuration {

    pub struct Config {
        input_filename: String,
        output_filename: Option<String>
    }


    impl Config {
        pub fn new(args: &Vec<String>) -> Result<Self, String>{
            let input = args.get(1).expect("Source file needed");
/*
            match args.get(2) {
                Some(option) => {
                    if option == "-o" {

                    }
                }
                None => 
            }
*/
            let binary = args.get(3).expect("Binary name needed.");

            let arguments = Self {
                input_filename: input.clone(),
                output_filename: Some(binary.clone()),
            };

            Ok(arguments)
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let _config = Config::new(&args).unwrap_or_else(|e: String|{ panic!("{}", e)});


    println!("Hello Oberust!")
}