// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

// https://doc.rust-lang.org/std/sync/mpsc/index.html
/*

Ce module fournit une communication basée sur des messages sur des canaux, concrètement définis parmi trois types :

    émetteur
    émetteur synchrone
    Récepteur

Un expéditeur ou un expéditeur synchrone est utilisé pour envoyer des données à un récepteur.
// Les deux expéditeurs peuvent être clonés (multiproducteurs), de sorte que plusieurs threads peuvent envoyer simultanément des données à un récepteur (consommateur unique).

*/


use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);


    let txa = Arc::new(tx); //Si seul : move occurs because `txa` has type `Arc<Sender<u32>>`, which does not implement the `Copy` trait
    let tx1 = Arc::clone(&txa);
    let tx2 = Arc::clone(&txa); // il en faut un second car sinon "value used here after move"

    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap(); //ici sinon "value used here after move"
            thread::sleep(Duration::from_secs(1));
        }
    });
}



//LES TESTS



#[test]
fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
