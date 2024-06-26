#![allow(unused)]

mod module_02;
mod module_03;
mod module_04;
mod module_05;
mod module_06;

use module_02::collection_fundamentals;
use module_03::working_with_sequences;
use module_03::working_with_sequences_v2;
use module_03::working_with_sequences_v3;
use module_04::using_maps_and_sets;
use module_04::using_maps_and_sets_v2;
use module_05::working_with_strings;
use module_06::managing_collections_in_memory;
use module_06::managing_collections_in_memory_demo;

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
    // using_maps_and_sets_v2::demo_hashset_and_btreeset();

    // Module 05 - Working With Strings
    // working_with_strings::introduction_to_strings();
    // working_with_strings::useful_string_functions();
    // working_with_strings::primitive_string();
    // working_with_strings::demo_strings();

    // Module 06 - Managing Collections in Memory
    // managing_collections_in_memory::collections_and_memory_management();
    // managing_collections_in_memory::moving_copying_and_cloning_collections();
    // managing_collections_in_memory_demo::demo_cloning_and_copying_collections();
    managing_collections_in_memory_demo::demo_capacity_management();
}
