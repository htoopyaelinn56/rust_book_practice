fn main(){
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third = v.get(200);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let mut v = vec![100, 32, 57];

    let borrowed = &v[0];
    v.push(88);

    /// println!("{borrowed}"); <- occurs compile time error

    /// mutable iteration
    for i in v.iter_mut() {
        *i *= -1;
    }


    for i in &mut v {
        *i *= -1;
    }

    println!("{:?}", v);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}