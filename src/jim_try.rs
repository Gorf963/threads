use std::thread;
use std::time;
use std::sync::mpsc;
use std::sync::mpsc::{Sender};//, Receiver};


static NTHREADS: i32 = 10;


pub fn jim_try() {

    let mut children = Vec::new();
    //let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();    
    let (tx,rx) = mpsc::channel();


    for i in 0..NTHREADS {
        let thread_tx = tx.clone();

        let child = thread::spawn(move||threader(i, thread_tx));
        children.push(child);
    }

    while children.len()>0 {
        let return_string = rx.recv().unwrap();
        println!("{:?}", return_string);
        if return_string == "End"
        {
            children.remove(0);
        }
    }
}

fn threader(a: i32, thread_tx: Sender<String>) {
    
    for i in 1..100 {
        let s: String;    
        s = format!("Thread {}, Print {}",a,i);

        thread_tx.send(s)
        .expect("Thread issue");
        
        thread::sleep(time::Duration::from_millis(100));
    }
    let s = format!("End");
    thread_tx.send(s).expect("Thread issue");
}