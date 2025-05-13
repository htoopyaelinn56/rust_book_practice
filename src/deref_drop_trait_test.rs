use std::fmt::{Display, Formatter};
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox");
    }
}
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    println!("y = {}", *y);
    drop(y);
    println!("Dropped y manually");

    println!("end of main fn");
}
