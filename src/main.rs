#![allow(unused)]

mod module_02;
mod module_03;
mod module_04;

use module_02::collection_fundamentals;
use module_03::working_with_sequences;
use module_03::working_with_sequences_v2;
use module_03::working_with_sequences_v3;
use module_04::using_maps_and_sets;
use module_04::using_maps_and_sets_v2;

fn main() {
    // Module 02 - Collection Fundamentals
    // collection_fundamentals::practical_collections();
    // collection_fundamentals::tuples_arrays_and_slices();
    // collection_fundamentals::demo_collection_fundamentals();

    // Module 03 - Working With Sequences
    // working_with_sequences::useful_vec_functions();
    // working_with_sequences::demo_vec();
    // working_with_sequences_v2::demo_vecdeque();
    // working_with_sequences_v3::demo_linkedlist();

    // Module 04 - Using Maps and Sets
    // using_maps_and_sets::demo_hashmap_and_btreemap();
    // using_maps_and_sets::useful_map_functions();
    // using_maps_and_sets::common_hashmap_operations();
    // using_maps_and_sets::btreemap_common_operations();
    // using_maps_and_sets::useful_hashset_functions();
    // using_maps_and_sets::useful_btreeset_functions();
    // using_maps_and_sets::hashset_common_operations();
    // using_maps_and_sets::btreeset_common_operations();
    using_maps_and_sets_v2::demo_hashset_and_btreeset();
}
