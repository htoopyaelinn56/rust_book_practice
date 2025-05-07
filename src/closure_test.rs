use std::thread;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let printer = |x: &str| {
        println!("{}", x);
    };

    let adder = |x: i32, y: i32| x + y;

    printer("hello");
    let added = adder(3, 4);
    println!("{}", added);

    let mut list = vec![1, 2, 3];

    let mut borrows_mutably = |x| list.push(x);

    borrows_mutably(4);
    borrows_mutably(5);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || {
        println!("From thread: {list:?}");
        println!("it is from thread")
    })
    .join()
    .unwrap();

    let option = Option::Some(1);

    let v1 = option.unwrap_or_else(|| 0);
    let v2 = option.unwrap_or_else(|| 1);
    println!("{v1} , {v2}");

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}
