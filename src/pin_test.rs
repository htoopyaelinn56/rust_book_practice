use std::pin::pin;

struct MyStruct {
    data: String,
    pointer_to_data: *const u8, // points to data.as_ptr()
}

fn example_without_pin() {
    let my_data = "Hello, world!".to_string();
    let mut my_struct = MyStruct {
        data: my_data,
        pointer_to_data: std::ptr::null(),
    };

    // SAFETY: We can safely set the pointer after the struct is created
    my_struct.pointer_to_data = my_struct.data.as_ptr();
    println!(
        "BEFORE MOVED, pointer_to_data: {:?}, data: {:?}",
        my_struct.pointer_to_data,
        my_struct.data.as_ptr()
    );

    move_data(my_struct.data);
}

fn example_with_pin() {
    use std::pin::Pin;

    let my_data = "Hello, world!".to_string();
    let mut my_struct = MyStruct {
        data: my_data,
        pointer_to_data: std::ptr::null(),
    };

    // SAFETY: We can safely set the pointer after the struct is created
    my_struct.pointer_to_data = my_struct.data.as_ptr();
    println!(
        "BEFORE PINNED, pointer_to_data: {:?}, data: {:?}",
        my_struct.pointer_to_data,
        my_struct.data.as_ptr()
    );

    let pinned_struct = Pin::new(&mut my_struct);
    
    // compile-time error
    // move_data(pinned_struct.data)
}

fn move_data(my_data: String)  {
   println!("Moved data is at {:?}", my_data.as_ptr());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_referential() {
        example_without_pin();
    }
}
