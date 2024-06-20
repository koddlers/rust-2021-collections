pub mod working_with_sequences {
    pub fn useful_vec_functions() {
        // initialization using the `vec!` macro
        let vec = vec![0, 1, 2, 3, 4];
        println!("vec: {:?}", vec);

        // initializing a vector with a given size and with a uniform value
        let uniform = vec![0; 6];    // creates a vector of size 6 where each element is `0`
        println!("uniform: {:?}", uniform);

        // initialization with `new`
        let mut new_vec = Vec::new();
        new_vec.push(1);
        new_vec.push(3);
        new_vec.push(5);
        println!("new_vec: {:?}", new_vec);

        let vec_with_capacity: Vec<i32> = Vec::with_capacity(5);
        println!("vec_with_capacity: {:?}, with capacity: {:?}",
                 vec_with_capacity, vec_with_capacity.capacity());

        let from_vec = Vec::from([0, 1, 2]);
        println!("from_vec: {:?}\n", from_vec);

        // Indexed access
        let mut my_vec = vec![2, 4, 6];
        println!("my_vec (before): {:?}", my_vec);

        my_vec[0] = 123;
        println!("my_vec (after): {:?}\n", my_vec);

        let my_num = my_vec[2];
        println!("my_num: {my_num}");

        let index = 0;
        let val = my_vec.get(index);
        match val {
            None => println!("Nothing at index {index}\n"),
            Some(v) => println!("Found value: {v}, at index: {index}\n")
        }

        let mut_val = my_vec.get_mut(index);
        // println!("mut_val: {}", mut_val.unwrap());
        match mut_val {
            None => println!("Nothing at index {index}"),
            Some(x) => {
                println!("Found value: {x}, at index: {index}");
                println!("Changing value at index: {index}");
                *x = 1024;
                println!("New value at index: {index}, is {x}");
            }
        }
        println!("my_vec (mutated): {:?}\n", my_vec);

        // using vectors as stack
        let mut stack = vec![2, 4, 6];
        println!("stack (before): {:?}", stack);

        stack.push(8000);
        println!("stack (after a push): {:?}", stack);

        let num = stack.pop();
        match num {
            None => println!("Nothing to pop"),
            Some(n) => println!("Popped number: {n} from the stack")
        }
        println!("stack (after the pop): {:?}\n", stack);

        // general utility methods
        let mut vec_utils = vec![2, 4, 6];
        println!("vec_utils: {:?}", vec_utils);
        println!("Length of `vec_utils` is: {}\n", vec_utils.len());

        // appending at the end of the vector
        vec_utils.append(&mut vec![1, 2, 3]);
        println!("vec_utils (after append): {:?}\n", vec_utils);

        // clearing the vector
        vec_utils.clear();
        println!("vec_utils (after clear): {:?}\n", vec_utils);

        // draining the vector
        vec_utils.append(&mut vec![1, 2, 3, 4, 5, 6, 7]);
        println!("vec_utils (before draining): {:?}", vec_utils);
        vec_utils.drain(3..);
        println!("vec_utils (after draining): {:?}\n", vec_utils);

        // inserting into the vector
        vec_utils.insert(0, 500000);
        println!("vec_utils (after insertion): {:?}\n", vec_utils);

        // removing from the vector
        vec_utils.remove(0);
        println!("vec_utils (after removal): {:?}\n", vec_utils);

        // `retain()` keeps the elements that match the criteria given by the passed in closure
        vec_utils.clear();
        vec_utils.append(&mut vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        vec_utils.retain(|&n| n % 2 == 0);  // keep all the even elements
        println!("vec_utils.retain(): {:?}", vec_utils);

        // retain with mutation allows mutation of each element as we filter
        vec_utils.retain_mut(
            |n| if *n > 4 {
                *n += 100;
                true
            } else {
                false
            });
        println!("vec_utils.retain_mut(): {:?}\n", vec_utils);

        // truncate(): truncates from the len index specified
        vec_utils.append(&mut vec![400, 500, 600]);
        println!("vec_utils.truncate() (before): {:?}", vec_utils);
        vec_utils.truncate(2);  // tarts from index 2 and truncates the rest
        println!("vec_utils.truncate() (after): {:?}\n", vec_utils);
    }
}