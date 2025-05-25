mod variable_test;
mod tuple_test;
mod loop_test;
mod ownership_test;
mod struct_test;
mod struct_test_2;
mod enum_test;
mod module_test;
pub mod module_test_2;
mod vector_test;
mod string_test;
mod hashmap_test;
mod error_handling_test;
mod generics_test;
mod trait_test;
mod lifetime_test;
mod test_cases;
mod two_pointer_remove_duplicates;
mod closure_test;
mod iterator_test;
mod linked_list;
mod deref_drop_trait_test;
mod reference_counting_test;
mod graph_datastructure;
mod ref_cell_test;
mod memory_leak_test;
mod thread_test;
mod message_passing;

fn main() {
    module_test::call();
}
