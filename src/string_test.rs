fn main(){
    let mut s = String::from("hello world");
    s.push_str("world");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    let hello = "Hello";

    let s = &hello[0..1];
    println!("{}", s);
}