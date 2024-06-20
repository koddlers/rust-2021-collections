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
}