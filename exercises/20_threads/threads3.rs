use std::{sync::{mpsc, Arc, Mutex}, thread, time::Duration};

struct Queue {
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Self {
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Arc<Queue>, tx: Arc<Mutex<mpsc::Sender<u32>>>) {
    let tx1 = Arc::clone(&tx);
    let q1 = Arc::clone(&q);
    thread::spawn(move || {
        for val in q1.first_half.iter() {
            println!("Sending {val:?}");
            tx1.lock().unwrap().send(*val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    let tx2 = Arc::clone(&tx);
    let q2 = Arc::clone(&q);
    thread::spawn(move || {
        for val in q2.second_half.iter() {
            println!("Sending {val:?}");
            tx2.lock().unwrap().send(*val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threads3() {
        let (tx, rx) = mpsc::channel();
        let queue = Arc::new(Queue::new());

        send_tx(queue, Arc::new(Mutex::new(tx)));

        let mut received = Vec::with_capacity(10);
        for value in rx {
            received.push(value);
        }

        received.sort();
        assert_eq!(received, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
