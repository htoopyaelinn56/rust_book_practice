use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main(){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");

    println!("---");

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi (thread1)"),
            String::from("from (thread1)"),
            String::from("the (thread1)"),
            String::from("thread (thread1)"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let tx2 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi (thread2)"),
            String::from("from (thread2)"),
            String::from("the (thread2)"),
            String::from("thread (thread2)"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}