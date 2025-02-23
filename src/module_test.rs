use crate::module_test_2;
pub fn call() {
    let mut s = module_test_2::ExampleStruct {
        text: String::from("This is a test"),
    };

    println!("{}", s.text);

    s.modify("This is a test edited");

    println!("{}", s.text);
}
