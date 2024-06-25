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

    pub fn useful_string_functions() {
        let mut string = String::from("Hello");
        println!("string: {}", string);

        let bytes = string.as_bytes();
        println!("bytes: {:?}", bytes);

        let slice = string.as_str();
        println!("slice: {}", slice);

        string.truncate(3);     // truncates everything starting from specified index
        println!("string (after `truncate()`): {}", string);

        string.pop();
        println!("string (after `pop()`): {}", string);

        string = String::from("Hello");
        // splits at specified index and returns the remaining string inclusive of the
        // character at the specified index
        let lo = string.split_off(3);
        println!("lo: {}", lo);

        string = String::from("Hello");
        string.push_str(" World");
        println!("string (after `push_str()`): {}", string);

        string.insert_str(0, "Well, ");
        println!("string (after `insert_str()`): {}", string);
    }

    pub fn primitive_string() {
        let foobar = "foobar";
        println!("foobar: {}", foobar);

        let ptr = foobar.as_ptr();
        println!("ptr: {:?}", ptr);

        let char_iter = foobar.char_indices();
        println!("char_iter: {:?}", char_iter);
        println!("char_iter.as_str(): {:?}", char_iter.as_str());

        let foo = foobar.find("foo");
        match foo {
            None => println!("Couldn't find what you searched for"),
            Some(idx) => println!("Found `foo` at index {idx}")
        }
        println!("foo: {:?}", foo);

        let foo1 = foobar.get(0..4);
        println!("foo1: {:?}", foo1);

        let trimmed = foobar.trim();
        println!("trimmed: {trimmed}");

        let three_times = foobar.repeat(3);
        println!("three_times: {}", three_times);
    }

    pub fn demo_strings() {
        // Literal, primitive string versus a String
        let primitive_greeting: &str = "Hello World";
        let mut greeting = String::from(primitive_greeting);
        println!("{}", primitive_greeting);
        println!("{}\n", greeting);

        // Common String and primitive string (&str) operations

        // Accessing characters by index

        // Don't do this!
        // println!("First char in greeting: {}\n", greeting[0]);

        let first_char = greeting.as_str().chars().nth(0);
        println!("First char in greeting: {:?}", first_char);

        let str_bytes = greeting.as_bytes();
        // Now you can index by integer if you want...
        println!("First char in greeting (bytes): {:?}\n", str_bytes[0]);

        // Adding to a String
        greeting.push_str(", my name is Shaphil");
        println!("{}", greeting);

        // Removing a character from a String
        greeting.remove(0);
        println!("{}", greeting);

        // Treating a String like a stack
        greeting.pop();
        println!("{}\n", greeting);

        greeting.push('l'); // Note the single quotes for a char
        println!("{}", greeting);

        // Inserting characters by index
        greeting.insert(0, 'H');
        println!("{}", greeting);

        // Inserting a string primitive by index
        greeting.insert_str(0, "Well, ");
        println!("{}\n", greeting);

        // Performing find and replace on a String
        let substr = "Hello World";
        let hello_world_start = greeting.find("Hello World").unwrap_or(greeting.len());
        let hello_world_end = hello_world_start + substr.len();

        greeting.replace_range(hello_world_start..hello_world_end, "hello world");
        println!("{}\n", greeting);

        // Lowercase/Uppercase
        greeting.make_ascii_lowercase();
        println!("Lowercase greeting: {}", greeting);

        greeting.make_ascii_uppercase();
        println!("Uppercase greeting: {}\n", greeting);

        // string to integer
        let maybe_number = "5000".parse::<u32>();
        println!("Number: {:?}", maybe_number);

        let maybe_eleven = "eleven".parse::<u32>();
        println!("Error: {:?}\n", maybe_eleven);

        // Trimming a string
        let str_with_spaces = "   Hello World        ";
        println!("Trimmed str: {}", str_with_spaces.trim());
        println!("Trimmed end str: {}", str_with_spaces.trim_end());
        println!("Trimmed start str: {}\n", str_with_spaces.trim_start());

        // Matching
        println!("Greeting: {}", greeting);
        println!("Does greeting start with 'Hello': {}\n", greeting.starts_with("Hello"));

        // Removing characters by match
        println!("Greeting: {}", greeting);
        println!("Trimmed end by match: {}\n", greeting.trim_end_matches("SHAPHIL"));

        let repeating = "11011";
        println!("Trimmed end by match: {}", repeating.trim_end_matches("1"));
        println!("Trimmed start by match: {}\n", repeating.trim_start_matches("1"));

        // Going from a String to a Vec
        // Splitting by character
        println!("Split greeting: {:?}", greeting.split(',').collect::<Vec<&str>>());

        // Splitting by str
        println!("Split greeting: {:?}", greeting.split(", HELLO WORLD,").collect::<Vec<&str>>());

        // Splitting by function
        println!("Split greeting: {:?}", greeting.split(char::is_uppercase).collect::<Vec<&str>>());

        // Splitting at a specific index
        println!("{:?}\n", greeting.split_at(4));
    }
}