fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    let mut s3 = takes_ownership(s2);
    borrow_ownership(&mut s3);

    println!("{s3}");

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");

    let mut text  = String::from("hello world");
    let first_word = first_word(&text);
    // text.clear(); <-- occurs compile time error
    println!("{first_word}");
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}

fn borrow_ownership(some_string: &mut String) {
    some_string.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}