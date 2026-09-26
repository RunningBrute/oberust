mod configuration {

    pub struct ProgramArguments {
        input_file_name: String,
        output_file_name: Option<String>
    }


    pub fn parse_args(args: &[&str]) -> Result<ProgramArguments, String>{
        let arguments = ProgramArguments {
            input_file_name: String::from("empty"),
            output_file_name: Some(String::from("empty"))
        };

        Ok(arguments)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("Hello Oberust!")
}