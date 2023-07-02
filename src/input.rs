pub fn get_numeric_input(command: &str) -> usize {
    loop {
        println!("{} ", command);

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        // println!("Received input: {}", &line);

        let parsed_result: Result<i32, _> = line.trim().parse();

        match parsed_result {
            Ok(parsed_value) => {
                if parsed_value > 0 && parsed_value <= 7 {
                    return parsed_value as usize;
                }
            }
            Err(_error) => {
                // println!("Error: {:?}", error)
            }
        };

        println!("Invalid input, try again!");
    }
}
