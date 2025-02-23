
pub struct ExampleStruct {
    pub text: String,
}

impl ExampleStruct {
    pub fn modify(&mut self, text: &str) {
        self.text = String::from(text);
    }
}

pub mod hosting {

    pub fn add_to_waitlist() {
        super::some_function_at_root();
    }

    fn seat_at_table() {}
}

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}

pub fn some_function_at_root() {}