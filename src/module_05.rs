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
}