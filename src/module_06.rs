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