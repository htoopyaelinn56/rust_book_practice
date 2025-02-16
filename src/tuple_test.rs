fn main(){
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);
    let x = {
        3
    };
}

fn condition_fn() -> () {
    let condition = true;
    let number = if condition { 5 } else { 1 };

    println!("The value of number is: {number}");
}