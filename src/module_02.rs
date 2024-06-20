pub mod collection_fundamentals {
    use std::collections::HashMap;

    pub fn practical_collections() {
        // Simple examples of vectors/strings
        println!("Simple examples of vectors/strings:");
        // let mut my_vec = Vec::from([2, 4, 6]);
        let mut my_vec = vec![2, 4, 6];     // alternative syntax
        my_vec.push(8);
        println!("my_vec: {:?}", my_vec);

        let mut greeting = String::from("Hello");
        greeting = greeting + ", World";
        println!("greeting: {}", greeting);

        // simple examples of the hashmap
        println!("\nSimple examples of the hashmap:");
        let mut coffee_ratings = HashMap::new();
        coffee_ratings.insert("Latte", 10);
        coffee_ratings.insert("Cappuccino", 9);

        for (name, rating) in &coffee_ratings {
            println!("{name} rating: {rating}");
        }
    }

    pub fn tuples_arrays_and_slices() {
        // tuples can hold many types of data in itself
        let tuple = (1, 2, "Hello World", 3.14, false, vec![1, 2, 3], [4, 5, 6]);
        println!("tuple: {:?}", tuple);

        let array = [1, 3, 5, 7, 9, 11, 13, 17, 19];
        println!("array: {:?}", array);

        let slice = &array[1..3];
        println!("slice: {:?}", slice);
    }

    pub fn demo_collection_fundamentals() {
        /*
        * High-level, Practical Collections
        * Vec, HashMap, String
        */

        // Vec
        // Initialization and basic usage
        let mut prime_numbers = vec![2, 3, 5, 7];
        println!("prime_numbers: {:?}\n", prime_numbers);

        let mut even_numbers = Vec::from([2, 4, 6, 8]);
        println!("even_numbers: {:?}\n", even_numbers);

        let odd_numbers: Vec<i32> = Vec::with_capacity(10);
        println!("Capacity of `odd_numbers`: {:?}\n", odd_numbers.capacity());

        prime_numbers.push(11);
        println!("prime_numbers (after a push()): {:?}\n", prime_numbers);

        even_numbers.pop();
        println!("even_numbers (after a pop()): {:?}\n", even_numbers);

        // HashMap
        let mut my_string_map = HashMap::new();
        my_string_map.insert(1, "Hello");
        my_string_map.insert(2, "World");
        println!("my_string_map: {:?}\n", my_string_map);
        // for (key, value) in my_string_map {
        //     println!("key: {}, value: {}", key, value);
        // }

        let my_coffee_map = HashMap::from([
            ("Drip", 2.99),
            ("Espresso", 4.50)
        ]);
        println!("my_coffee_map: {:?}\n", my_coffee_map);

        let init_capacity_map: HashMap<i32, &str> = HashMap::with_capacity(10);
        println!("Capacity of `init_capacity_map`: {:?}\n", init_capacity_map.capacity());

        // String
        let greetings = String::from("Hello again world");
        println!("greetings: {}\n", greetings);

        let mut string_with_capacity = String::with_capacity(5);
        println!("string_with_capacity.capacity() (before): {}\n", string_with_capacity.capacity());

        for _ in 0..6 {
            string_with_capacity.push('S');
        }
        println!("string_with_capacity: {}", string_with_capacity);
        println!("string_with_capacity.capacity() (after): {}\n", string_with_capacity.capacity());

        /*
         * Primitive Collections
         * Tuple, Array, Slice
         */

        // Tuple
        let tuple = ('A', 1, "Hello");
        println!("Tuple: {:?}\n", tuple);

        // tuple de-structuring
        let (character, integer, string) = tuple;
        println!("character: {character}");
        println!("integer: {integer}");
        println!("string: {string}\n");

        assert_eq!(tuple.0, 'A');
        assert_eq!(tuple.1, 1);
        assert_eq!(tuple.2, "Hello");

        // Array
        let mut array = [1, 2, 3, 4];
        array[1] = 1001;
        println!("array: {:?}\n", array);

        // Slice
        let slice = &array[1..3];
        println!("slice: {:?}\n", slice);

        let mutable_slice = &mut array[1..3];
        mutable_slice[0] = 5001000;
        println!("mutable_slice: {:?}\n", mutable_slice);

        return ();
    }
}