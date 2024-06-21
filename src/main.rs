#![allow(unused)]

mod module_02;
mod module_03;

use module_02::collection_fundamentals;
use module_03::working_with_sequences;
use module_03::working_with_sequences_v2;

fn main() {
    // Module 02 - Collection Fundamentals
    // collection_fundamentals::practical_collections();
    // collection_fundamentals::tuples_arrays_and_slices();
    // collection_fundamentals::demo_collection_fundamentals();

    // Module 03 - Working With Sequences
    // working_with_sequences::useful_vec_functions();
    // working_with_sequences::demo_vec();
    working_with_sequences_v2::demo_vecdeque();
}
