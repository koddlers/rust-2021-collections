pub mod working_with_strings {
    pub fn introduction_to_strings() {
        let primitive_str = "Hello World";
        let mut dynamic_string = String::new();
        let mut chars: Vec<char> = primitive_str.chars().collect();

        for c in chars {
            dynamic_string.push(c);
            // println!("{}", c);
        }
        println!("dynamic_string: {}", dynamic_string);

        let hello = dynamic_string.strip_suffix(" World");
        match hello {
            None => println!("Suffix not found"),
            Some(val) => println!("After removing suffix: {}", val)
        }
    }
}