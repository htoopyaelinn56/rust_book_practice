use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let item = Rc::new(1);
    println!("ref_count: {} item = {}", Rc::strong_count(&item), &item);
    // *item = 5; will get error
    perform(&item);
    println!("ref_count at the end of func: {}", Rc::strong_count(&item));

    println!("-----");

    let item = Rc::new(RefCell::new(1));
    println!("ref_count: {} item = {}", Rc::strong_count(&item), item.borrow());
    *item.borrow_mut() = 5;
    *item.borrow_mut() += 1;
    println!("ref_count: {} item after modified = {}", Rc::strong_count(&item), item.borrow());
    perform_by_ref_cell(&item);
    println!("ref_count at the end of func: {}", Rc::strong_count(&item));
}

fn perform(arg : &Rc<i32>) {
    let second = Rc::clone(&arg);
    println!("ref_count when second: {}, value = {}", Rc::strong_count(&second), &second);
    let third = Rc::clone(&arg);
    println!("ref_count when third: {}, value = {}", Rc::strong_count(&third), &third);
}

fn perform_by_ref_cell(arg : &Rc<RefCell<i32>>) {
    let second = Rc::clone(&arg);
    println!("ref_count when second: {}, value = {}", Rc::strong_count(&second), second.borrow());
    let third = Rc::clone(&arg);
    println!("ref_count when third: {}, value = {}", Rc::strong_count(&third), third.borrow());
}