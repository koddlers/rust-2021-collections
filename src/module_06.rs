pub mod managing_collections_in_memory {
    pub fn collections_and_memory_management() {
        let mut vector = Vec::from([1, 2, 3]);
        println!("Capacity: {}", vector.capacity());

        vector.push(4);
        println!("Capacity: {}", vector.capacity());
    }
}