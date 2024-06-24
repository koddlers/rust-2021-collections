pub mod using_maps_and_sets {
    use std::cmp::Ordering;
    use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

    // Hash, Eq, PartialEq needed to use this struct as the custom key to a HashMap
    #[derive(Debug, PartialEq, Hash, Eq)]
    struct Coffee {
        id: i32,
        count: i32,
    }

    // Custom Ord, PartialOrd needed for the BTreeMap to sort by count
    impl Ord for Coffee {
        fn cmp(&self, other: &Self) -> Ordering {
            self.count.cmp(&other.count)
        }
    }

    impl PartialOrd for Coffee {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.count.cmp(&other.count))
        }
    }

    fn generate_coffee() -> Coffee {
        Coffee {
            id: 10000,
            count: 50,
        }
    }

    pub fn demo_hashmap_and_btreemap() {
        let mut coffee_map = HashMap::from([
            ("Coffee1", Coffee { id: 1000, count: 10 }),
            ("Coffee2", Coffee { id: 2000, count: 40 }),
            ("Coffee3", Coffee { id: 3000, count: 500 })
        ]);

        // HashMap element access and insertion
        coffee_map.insert("Coffee4", Coffee { id: 4000, count: 1 });
        println!("Coffee4: {:?}\n", coffee_map.get("Coffee4"));
        // println!("coffee_map: {:?}", coffee_map);

        coffee_map.insert("Coffee4", Coffee { id: 5000, count: 99999 });
        println!("Coffee4: {:?}\n", coffee_map.get("Coffee4"));
        // println!("coffee_map: {:?}", coffee_map);

        // Entry API
        // Only insert if the key doesn't already exist...
        coffee_map.entry("Coffee4").or_insert(Coffee { id: 5000, count: 1 });
        println!("Coffee4: {:?}\n", coffee_map.get("Coffee4"));
        // println!("coffee_map: {:?}", coffee_map);

        // Do the same thing but using a custom function
        coffee_map.remove("Coffee4");
        coffee_map.entry("Coffee4").or_insert_with(generate_coffee);
        println!("Coffee4: {:?}\n", coffee_map.get("Coffee4"));
        // println!("coffee_map: {:?}", coffee_map);

        // Using a custom type as a key
        let mut custom_key_map = HashMap::from([
            (Coffee { id: 1000, count: 5 }, "Coffee1"),
            (Coffee { id: 2000, count: 2 }, "Coffee2"),
            (Coffee { id: 3000, count: 1 }, "Coffee3"),
        ]);

        println!("Coffee1: {:?}\n", custom_key_map.get(&Coffee { id: 1000, count: 5 }));
        println!("Coffee1: {:?}\n", custom_key_map.get(&Coffee { id: 1000, count: 0 }));

        // Iterating over the key/value pairs of a HashMap
        for (coffee, name) in &custom_key_map {
            println!("{coffee:?} / {name}");
        }
        println!();

        // Filtering a HashMap
        custom_key_map.retain(|coffee, _name| coffee.count < 5);
        println!("After filtering: {:?}\n", custom_key_map);

        // BTreeMap
        let mut coffee_descriptions = BTreeMap::from([
            (Coffee { id: 1000, count: 5 }, "Bold, rich flavor"),
            (Coffee { id: 2000, count: 2 }, "Ethiopian blend"),
            (Coffee { id: 3000, count: 1 }, "Medium, fruity"),
        ]);
        println!("BTreeMap sorting: {:?}\n", coffee_descriptions);

        // Note: BTreeMap has very similar methods to HashMap
        // Remember, sorting is of great value if you are using a BTreeMap
        println!("First pair: {:?}\n", coffee_descriptions.first_key_value());
        println!("Last pair: {:?}\n", coffee_descriptions.last_key_value());

        coffee_descriptions.pop_first();
        coffee_descriptions.pop_last();
        println!("After removal: {:?}\n", coffee_descriptions);

        coffee_descriptions.insert(Coffee { id: 6000, count: 50 }, "Dark, nutty");
        println!("After insertion: {:?}\n", coffee_descriptions);
    }

    pub fn useful_map_functions() {
        // HashMap::new()
        let mut new_map = HashMap::new();

        new_map.insert("Potato", 35.0);
        new_map.insert("Rice", 85.0);
        new_map.insert("Chicken", 285.0);
        new_map.insert("Beef", 1000.0);

        for (item, price) in new_map.clone() {
            println!("{item} costs {price:.2} tk");
        }
        println!();

        // HashMap::from()
        let mut from_map = HashMap::from([
            ("Latte", 320.0),
            ("Honey Latte", 350.0),
            ("Cappuccino", 280.0),
            ("Black Coffee", 120.0)
        ]);

        for (coffee, cost) in from_map.clone() {
            println!("Coffee: {coffee} costs {cost} tk");
        }
        println!();

        // HashMap::with_capacity()
        let mut cap_map: HashMap<&str, f64> = HashMap::with_capacity(5);

        println!("`cap_map` capacity (before): {}", cap_map.capacity());
        cap_map.extend(new_map);
        cap_map.extend(from_map);
        println!("`cap_map` capacity (after): {}", cap_map.capacity());
        println!();

        println!("items in the `cap_map`");
        for (key, val) in cap_map.clone() {
            println!("{key}::{val}");
        }
        println!();

        // `BTreeMap` has only `new` and `from`
        // BTreeMap::new()
        let mut new_btr = BTreeMap::new();

        new_btr.insert(1, "January");
        new_btr.insert(2, "February");
        new_btr.insert(3, "March");
        new_btr.insert(4, "April");
        new_btr.insert(5, "May");
        new_btr.insert(6, "June");
        new_btr.insert(7, "July");
        new_btr.insert(8, "August");
        new_btr.insert(9, "September");
        new_btr.insert(10, "October");
        new_btr.insert(11, "November");
        new_btr.insert(12, "December");

        for (num, mon) in new_btr.clone() {
            println!("{mon} is the month number \t{num}");
        }
        println!();

        // BTreeMap::from()
        let mut from_btr = BTreeMap::from([
            ("Latte", 320.0),
            ("Honey Latte", 350.0),
            ("Cappuccino", 280.0),
            ("Black Coffee", 120.0),
            ("Mocha", 150.0)
        ]);

        for (coffee, cost) in from_btr.clone() {
            println!("Coffee {coffee} costs {cost:.2} TK");
        }
        println!();
    }

    pub fn common_hashmap_operations() {
        // Operations
        // let mut map = HashMap::new();
        // map.clear();
        // map.len();
        // map.insert("key", "value");
        // map.get("key");     // returns an `Option`
        // map.get_mut("key"); // returns an `Option` that can mutate
        // map.remove("key");
        // map.keys();     // returns an iterator that allows looping over the map keys randomly
        // map.values();   // returns an iterator that allows looping over the map values randomly
        // map.values_mut();   // returns an iterator that allows looping over the map values
        //                        randomly while allowing mutation
        // map.into_keys().collect();  // 1
        // map.into_values().collect();    // this and 1 does the same as the above functions,
        //                                    but they cause the map to be unusable
        // map.get_key_value("key");   // returns the key-value pair from the key as Option

        let mut my_map: HashMap<&str, f32> = HashMap::new();

        my_map.clear();
        println!("(Before) `my_map` has a capacity of {}", my_map.len());

        // if the key already exists in the map, `insert()` will update the value of the key
        my_map.insert("Latte", 3.50);
        println!("(After) `my_map` has a capacity of {}", my_map.len());

        let koffee = "Latte";
        let coffee = my_map.get(koffee);
        match coffee {
            None => println!("No koffee"),
            Some(price) => println!("koffee costs ${price:.2}"),
        }

        let mut_coffee = my_map.get_mut(koffee);
        match mut_coffee {
            None => println!("No KOFFEEEEE!"),
            Some(price) => {
                *price = 4.50;
                println!("New price of Koffee is ${price:.2}");
            }
        }
        println!("Checking for mutation: {:?}", my_map);
        println!("...AND the value is mutated\n");

        my_map.remove(koffee);
        println!("Checking for removal: {:?}", my_map);
        println!("yes the Koffee is gone\n");

        // re-populate
        my_map.insert("Potato", 35.0);
        my_map.insert("Rice", 85.0);
        my_map.insert("Chicken", 285.0);
        my_map.insert("Beef", 1000.0);

        println!("keys: {:?}", my_map.keys());
        println!("values: {:?}", my_map.values());
        println!("(key, value): {:?}", my_map.get_key_value("Potato"));
    }

    pub fn btreemap_common_operations() {
        let mut btree = BTreeMap::new();

        btree.insert("Potato", 35.0);
        btree.insert("Rice", 85.0);
        btree.insert("Chicken", 285.0);
        btree.insert("Beef", 1000.0);

        println!("{:?}", btree.first_entry());
        println!("{:?}", btree.first_entry().unwrap());
        println!();

        println!("{:?}", btree.last_entry());
        println!("{:?}", btree.last_entry().unwrap());
        println!();

        println!("btree (before): {:?}", btree);
        btree.pop_first();
        println!("btree (after): {:?}", btree);
        println!();

        println!("btree (before): {:?}", btree);
        btree.pop_last();
        println!("btree (after): {:?}", btree);
        println!();

        let mut other_map = BTreeMap::new();
        other_map.insert("test", 3.14);
        btree.append(&mut other_map);
        println!("btree (after append): {:?}", btree);
    }

    pub fn useful_hashset_functions() {
        // `HashSet` doesn't maintain insertion order
        let mut animals = HashSet::new();

        animals.insert("Lion");
        animals.insert("Tiger");
        animals.insert("Horse");
        animals.insert("Turtle");
        animals.insert("Tiger");
        animals.insert("Tiger");

        println!("There are {} items in the `animals` HashSet", animals.len());
        println!("Items in the `animals` set: {:?}\n", animals);

        let mut coffees = HashSet::from([
            "Latte",
            "Mocha",
            "Cappuccino",
            "Latte",
        ]);

        println!("There are {} items in the `coffees` HashSet", coffees.len());
        println!("Items in the `coffees` set: {:?}\n", coffees);

        let mut capaset: HashSet<i32> = HashSet::with_capacity(5);
        println!("capaset.len() (before): {}", capaset.len());
        println!("capaset.capacity() (before): {}", capaset.capacity());

        capaset.insert(1);
        capaset.insert(3);
        capaset.insert(5);
        capaset.insert(7);
        capaset.insert(9);
        capaset.insert(9);
        capaset.insert(9);

        println!("capaset.len() (before): {}", capaset.len());
        println!("capaset.capacity() (before): {}", capaset.capacity());
        println!("capaset: {:?}\n", capaset);
    }

    pub fn useful_btreeset_functions() {
        // BTreeSet are initialized the same way as HashSet,
        // except for the `with_capacity` method is not available
        let mut btree = BTreeSet::new();

        btree.insert("Palm");
        btree.insert("Mango");
        btree.insert("Lychee");
        btree.insert("Dates");
        btree.insert("Oranges");
        btree.insert("Oranges");
        btree.insert("Oranges");

        println!("btree.len(): {}", btree.len());
        println!("btree: {:?}\n", btree);

        let mut btree_from = BTreeSet::from([
            "Latte",
            "Mocha",
            "Cappuccino",
            "Latte",
            "Latte",
            "Latte",
        ]);

        println!("btree_from.len(): {}", btree_from.len());
        println!("btree_from: {:?}\n", btree_from);
    }

    pub fn hashset_common_operations() {
        let mut hashset = HashSet::new();

        hashset.insert("Latte");
        hashset.insert("Mocha");
        hashset.insert("Cappuccino");
        hashset.insert("Latte");
        hashset.insert("Latte");

        println!("hashset length: {}", hashset.len());  // get count of values in the set
        println!("hashset: {:?}\n", hashset);

        // remove all values from the set
        hashset.clear();
        println!("After `hashset.clear()`");
        println!("hashset length: {}", hashset.len());
        println!("hashset: {:?}\n", hashset);

        hashset.insert("Latte");
        hashset.insert("Cappuccino");
        println!("After `hashset.insert()`");
        println!("hashset length: {}", hashset.len());
        println!("hashset: {:?}\n", hashset);

        hashset.remove("Latte");
        println!("After `hashset.remove()`");
        println!("hashset length: {}", hashset.len());
        println!("hashset: {:?}\n", hashset);

        println!("hashset.contains(`Latte`): {}\n", hashset.contains("Latte"));

        // Common operations
        // let mut set1 = HashSet::new();
        // let mut set2 = HashSet::new();
        //
        // set1.difference(&set2);
        // set1.intersection(&set2);
        //
        // set1.is_subset(&set2);
        // set1.is_superset(&set2);
        // set1.is_disjoint(&set2);
        // set1.symmetric_difference(&set2);
        // set1.union(&set2);
    }

    pub fn btreeset_common_operations() {
        let mut btree = BTreeSet::new();

        btree.insert("Palm");
        btree.insert("Mango");
        btree.insert("Lychee");
        btree.insert("Dates");
        btree.insert("Oranges");
        btree.insert("Oranges");
        btree.insert("Oranges");

        println!("First element: {:?}", btree.first());
        println!("Last element: {:?}\n", btree.last());

        println!("btree (before: pop_first()): {:?}", btree);
        btree.pop_first();
        println!("btree (after: pop_first()): {:?}\n", btree);

        println!("btree (before: pop_last()): {:?}", btree);
        btree.pop_last();
        println!("btree (after: pop_last()): {:?}\n", btree);

        let mut other_set = BTreeSet::new();
        other_set.insert("Apples");

        println!("btree (before: append()): {:?}", btree);
        btree.append(&mut other_set);
        println!("btree (after: append()): {:?}\n", btree);
    }
}

pub mod using_maps_and_sets_v2 {
    use std::cmp::Ordering;
    use std::collections::{BTreeSet, HashSet};

    // Hash, Eq, PartialEq needed to use this struct as the custom key to a HashSet
    #[derive(Clone, Debug, PartialEq, Hash, Eq)]
    struct Coffee {
        id: i32,
        count: i32,
    }

    // Custom Ord, PartialOrd needed for the BTreeSet to sort by count
    impl Ord for Coffee {
        fn cmp(&self, other: &Self) -> Ordering {
            self.count.cmp(&other.count)
        }
    }

    impl PartialOrd for Coffee {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.count.cmp(&other.count))
        }
    }

    pub fn demo_hashset_and_btreeset() {
        // Note: There is a duplicated element here that isn't included
        let mut coffee_set = HashSet::from([
            Coffee { id: 1000, count: 10 },
            Coffee { id: 3000, count: 500 },
            Coffee { id: 2000, count: 40 },
            Coffee { id: 3000, count: 500 }
        ]);

        println!("Initial coffee set: {:?}\n", coffee_set);

        // HashSet element access and insertion
        coffee_set.insert(Coffee { id: 4000, count: 1 });

        // Fetching a value from a set
        println!("Coffee4: {:?}\n", coffee_set.get(&Coffee { id: 4000, count: 1 }));

        // Iterating over all values in a HashSet
        println!("All items in the hashset:");
        for coffee in &coffee_set {
            println!("{coffee:?}");
        }
        println!();

        // Most useful HashSet functions
        let set_a = HashSet::from([
            Coffee { id: 1000, count: 10 },
            Coffee { id: 2000, count: 20 },
            Coffee { id: 3000, count: 30 },
            Coffee { id: 4000, count: 40 }
        ]);

        let set_b = HashSet::from([
            Coffee { id: 1000, count: 10 },
            Coffee { id: 9999, count: 99 },
            Coffee { id: 8888, count: 88 },
            Coffee { id: 7777, count: 77 }
        ]);

        let difference: HashSet<&Coffee> = set_a.difference(&set_b).collect();
        println!("Difference: {}", difference.len());
        for item in difference.clone() {
            println!("{:?}", item);
        }
        println!();

        let intersection: HashSet<&Coffee> = set_a.intersection(&set_b).collect();
        println!("Intersection: {:?}\n", intersection);
        for item in intersection.clone() {
            println!("{item:?}");
        }
        println!();

        let set_is_disjoint: bool = set_a.is_disjoint(&set_b);
        println!("Disjoint?: {:?}\n", set_is_disjoint);

        let symmetric_diff: HashSet<&Coffee> = set_a.symmetric_difference(&set_b).collect();
        println!("Symmetric Difference: {}", symmetric_diff.len());
        for item in symmetric_diff {
            println!("{item:?}");
        }
        println!();

        let all_values: HashSet<&Coffee> = set_a.union(&set_b).collect();
        println!("Union: {}", all_values.len());
        for item in all_values {
            println!("{item:?}");
        }
        println!();

        // BTreeSet
        let mut coffee_tree_set = BTreeSet::from([
            Coffee { id: 3000, count: 1 },
            Coffee { id: 2000, count: 5 },
            Coffee { id: 1000, count: 2 }
        ]);

        println!("Unsorted HashSet:");
        for coffee in &coffee_set {
            println!("{coffee:?}");
        }
        println!();

        println!("BTreeSet sorting based on count: {}", coffee_tree_set.len());
        for item in &coffee_tree_set.clone() {
            println!("{:?}", item);
        }
        println!();

        // Note: BTreeSet has very similar methods to HashMap and HashSet
        // Remember, sorting is of great value if you are using a BTreeSet

        println!("First value: {:?}", coffee_tree_set.first());
        println!("Last value: {:?}\n", coffee_tree_set.last());

        coffee_tree_set.pop_first();
        coffee_tree_set.pop_last();
        println!("After removal: {:?}\n", coffee_tree_set);

        coffee_tree_set.insert(Coffee { id: 6000, count: 50 });
        coffee_tree_set.insert(Coffee { id: 7000, count: 0 });
        println!("After insertion: {:?}\n", coffee_tree_set);
    }
}