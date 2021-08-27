use rand::Rng;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

fn main() {
    let mut rng = rand::thread_rng();
    println!("Starting the Kitchen");
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();

    loop {
        let customers = rng.gen_range(0..10);
        println!("** number of customers: {}", customers);
        spawn_run_threads(&tx, customers);
        wait_tables(&rx, customers);
    }
}

fn spawn_run_threads(tx: &Sender<String>, customers: i32) {
    for i in 0..customers {
        let tx = tx.clone();
        thread::spawn(move || {
            let message = format!(">>Made customer's {} food", i);
            println!("{}", message);
            thread::sleep(Duration::from_millis(1000));
            tx.send(message)
        });
    }
}

fn wait_tables(rx: &Receiver<String>, customers: i32) {
    let mut left = 0;
    while left < customers {
        println!("///Serving Customers");
        thread::sleep(Duration::from_millis(200));
        let message = rx.try_recv();
        match message {
            Ok(m) => {
                println!("$$Recieved: {}", m);
                left += 1
            }
            _ => println!("Failure!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_run_threads() {}

    #[test]
    fn test_wait_tables() {}
}
