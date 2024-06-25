pub mod managing_collections_in_memory {
    pub fn collections_and_memory_management() {
        let mut vector = Vec::from([1, 2, 3]);
        println!("Capacity: {}", vector.capacity());

        vector.push(4);
        println!("Capacity: {}", vector.capacity());
    }

    pub fn moving_copying_and_cloning_collections() {
        // moving/cloning collections
        let vector = vec![2, 4, 6];
        let moved = vector;

        println!("moved: {moved:?}\n");
        // but cannot do this, since the ownership is moved
        // println!("vector: {vector:?}\n");

        // but if you borrow the value (prepending with a `&` character)
        // you can use both the variables pointing to the same data
        let vektor = vec![3, 5, 7];
        let movd = &vektor;

        println!("vektor: {vektor:?}");
        println!("movd: {movd:?}\n");

        let other = vec![9, 2, 8];
        // `clone()` makes a deep copy of the vector
        let cloned = other.clone();

        println!("other: {other:?}");
        println!("cloned: {cloned:?}");
    }
}

pub mod managing_collections_in_memory_demo {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Clone)]
    struct Coffee {
        id: i32,
        count: i32,
    }

    pub fn demo_cloning_and_copying_collections() {
        // Vec moving and cloning
        let coffees = Vec::from([
            Coffee { id: 1000, count: 10 },
            Coffee { id: 2000, count: 20 },
            Coffee { id: 3000, count: 30 }
        ]);
        println!("Vector of Coffees: {:?}", coffees);

        // Example 'move' - the vector is now owned by 'moved_coffees'
        let moved_coffees = coffees;
        println!("Vector of Coffees: {:?}\n", moved_coffees);

        // Uncommenting this will cause the code to not compile
        // Vecs are not Copyable since this would only copy the shallow, stack-based pointers.
        // This might result in multiple references to the same space of memory - not good!
        // Instead, we need to clone!
        // println!("Vector of Coffees: {:?}\n", coffees);

        // Example cloning (deep copy)
        let mut cloned_coffees = moved_coffees.clone();
        println!("Vector of Cloned Coffees: (before)\t {:?}", cloned_coffees);
        println!("Vector of Moved Coffees: (before)\t {:?}\n", moved_coffees);

        // The matching coffee in 'moved_coffees' is untouched.
        cloned_coffees[0].count = 10000;
        println!("Vector of Cloned Coffees: (after)\t {:?}", cloned_coffees);
        println!("Vector of Moved Coffees: (after)\t {:?}\n", moved_coffees);

        // HashMap moving and cloning
        let coffee_map = HashMap::from([
            ("Coffee1", Coffee { id: 1000, count: 10 }),
            ("Coffee2", Coffee { id: 2000, count: 40 }),
            ("Coffee3", Coffee { id: 3000, count: 500 })
        ]);
        println!("Coffee map:\t\t {:?}", coffee_map);

        // Example 'move' - the vector is now owned by 'moved_coffees'
        let moved_coffee_map = coffee_map;
        println!("Moved coffee map:\t {:?}\n", moved_coffee_map);

        // Uncommenting this will cause the code not to compile for the same reasons as Vec above!
        // println!("Coffee map after move: {:?}\n", coffee_map);

        // Example cloning a map (deep copy)
        let mut cloned_coffee_map = moved_coffee_map.clone();
        println!("HashMap of Cloned Coffees: (before)\t {:?}", cloned_coffee_map);
        println!("HashMap of Moved Coffees: (before)\t {:?}\n", moved_coffee_map);

        // The matching coffee in 'moved_coffee_map' is untouched.
        cloned_coffee_map.insert("Coffee1", Coffee { id: 1000, count: 0 });
        println!("HashMap of Cloned Coffees: (after)\t {:?}", cloned_coffee_map);
        println!("HashMap of Moved Coffees: (after)\t {:?}\n", moved_coffee_map);


        // String moving and cloning
        let my_string = String::from("Hello World");

        println!("Greeting: {}", my_string);

        // Example 'move' - the String is now owned by 'moved_string'
        let moved_string = my_string;

        // Uncommenting this will cause the code to not compile
        // println!("Greeting prior to string: {}\n", my_string);

        println!("Greeting after move: {}\n", moved_string);

        // Example cloning (deep copy)
        let mut cloned_string = moved_string.clone();
        println!("Cloned greeting: {:?}", cloned_string);
        println!("Moved greeting: {:?}\n", moved_string);

        // The matching str slice in 'moved_string' is untouched.
        let string_len = cloned_string.len();
        let find = cloned_string.find(" World");
        let world_offset = find.unwrap_or(string_len);
        cloned_string.replace_range(world_offset..string_len, "");

        println!("Cloned greeting after alteration: {:?}", cloned_string);
        println!("Moved greeting after alteration: {:?}\n", moved_string);
    }

    pub fn demo_capacity_management() {
        // Creating collections with capacity
        println!("Vec capacity management:");
        let mut coffees: Vec<Coffee> = Vec::with_capacity(100);
        println!("Capacity is {} | Length is {}", coffees.capacity(), coffees.len());

        for n in 1..76 {
            coffees.push(Coffee { id: n, count: n * 10 });
        };
        println!("Capacity is {} | Length is {}", coffees.capacity(), coffees.len());

        for n in 1..26 {
            coffees.push(Coffee { id: n, count: n * 10 });
        };
        println!("Capacity is {} | Length is {}", coffees.capacity(), coffees.len());

        for n in 1..26 {
            coffees.push(Coffee { id: n, count: n * 10 });
        };

        // On my machine, a new allocation occurs here - it over-allocates
        println!("Capacity is {} | Length is {}", coffees.capacity(), coffees.len());

        // Let's shrink it!
        coffees.shrink_to_fit();

        // This code is commented because the capacity does not change when we shrink it if we
        // call 'reserve()' just afterwards!

        // We know more coffees are coming in - let's make a new allocation happen beforehand!
        // coffees.reserve(100);
        // println!("Capacity is {} | Length is {}", coffees.capacity(), coffees.len());

        println!("Capacity is {} | Length is {}", coffees.capacity(), coffees.len());


        // HashMap capacity management!
        println!("\nHashMap capacity management:");
        let mut coffee_map: HashMap<String, Coffee> = HashMap::with_capacity(100);

        // Note that the capacity is more than we asked for!
        println!("Capacity is {} | Length is {}", coffee_map.capacity(), coffee_map.len());

        for n in 1..76 {
            coffee_map.insert(
                "Coffee".to_owned() + n.to_string().as_str(),
                Coffee { id: n, count: n * 10 },
            );
        };
        println!("Capacity is {} | Length is {}", coffee_map.capacity(), coffee_map.len());

        for n in 76..101 {
            coffee_map.insert(
                "Coffee".to_owned() + n.to_string().as_str(),
                Coffee { id: n, count: n * 10 },
            );
        };
        println!("Capacity is {} | Length is {}", coffee_map.capacity(), coffee_map.len());

        for n in 101..151 {
            coffee_map.insert(
                "Coffee".to_owned() + n.to_string().as_str(),
                Coffee { id: n, count: n * 10 },
            );
        };

        // On my machine, a new allocation occurs here!
        println!("Capacity is {} | Length is {}", coffee_map.capacity(), coffee_map.len());

        // Let's shrink it!
        coffee_map.shrink_to_fit();
        println!("Capacity is {} | Length is {}", coffee_map.capacity(), coffee_map.len());

        // We know more coffee entries are coming in - let's make a new allocation happen beforehand!
        coffee_map.reserve(100);
        println!("Capacity is {} | Length is {}", coffee_map.capacity(), coffee_map.len());
        // This call to reserve on the HashMap doubled its capacity on my system!
    }
}