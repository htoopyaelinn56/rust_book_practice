use std::rc::Rc;

fn main() {
    let item = Rc::new(1);
    println!("ref_count: {}", Rc::strong_count(&item));
    perform(&item);
    println!("ref_count at the end of func: {}", Rc::strong_count(&item));
}

fn perform(arg : &Rc<i32>) {
    let second = Rc::clone(&arg);
    println!("ref_count when second: {}", Rc::strong_count(&second));
    let third = Rc::clone(&arg);
    println!("ref_count when third: {}", Rc::strong_count(&third));
}