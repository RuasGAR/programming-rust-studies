use std::thread;
use std::sync::mpsc;
use std::time::Duration;
// MPSC -> Multiple producers, solo consumer;

fn main() {
    let (tx,rx) = mpsc::channel();
    
    // Thread de envio
    thread::spawn(move||{
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });


    //let received = rx.recv().unwrap();
    for received in rx {
        println!("Got {}",received);
    }
}
