use std::cell::RefCell;

fn main(){
    let item = RefCell::new(1);

    let mut item_1 = item.borrow_mut();
    *item_1 += 2;

    println!("item = {}", item_1);
}