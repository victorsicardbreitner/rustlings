// threads2.rs
//
// Building on the last exercise,
// we want all of the threads to complete their work.
//
// But this time 
// the spawned threads need to be in charge of 
// Updating a shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.

// https://doc.rust-lang.org/std/sync/struct.Mutex.html
// https://doc.rust-lang.org/stable/std/thread/struct.JoinHandle.html


use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 })); //ici on rajoute Mutex pour bloquer les threads qui attendent que le verrou devienne disponible. 
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // DONE: You must take an action before you update a shared value
            let mut counter = status_shared.lock().unwrap();
            counter.jobs_completed += 1; //Arc<T> automatically dereferences to T (via the Deref trait), so you can call T’s methods on a value of type Arc<T>. 
            //counter
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap(); // quand on commente cette ligne le décompte n'est pas effectué

        /*
        --------------le join() dans la documentation JoinHandle--------------
        Attend que le thread associé se termine.
        Cette fonction revient immédiatement si le thread associé est déjà terminé.
        En termes d'ordre atomique de la mémoire, l'achèvement du thread associé est synchronisé avec le retour de cette fonction.
        En d'autres termes, toutes les opérations effectuées par ce thread se produisent avant toutes les opérations qui se produisent après le retour de join.
         */

        // DONE: Print the value of the JobStatus.jobs_completed.
        // Did you notice anything interesting in the output?
        // Do you have to 'join' on all the handles?

        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }
}
