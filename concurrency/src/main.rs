use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Note:: execution of all spawned threads will stop when the main thread does,
    // and thus may not be allowed to finish! We will always count to 5 in the main
    // thread, and to some indeterminate number in the spawned thread here.
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn wait_on_threads_with_join_handles() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        // Calling join() on a JoinHandle will wait for its thread to finish
        // before returning a Result. If the spawned thread panics, the panic
        // message will be returned as an Error.
        handle.join().unwrap();
    }

    #[test]
    fn move_closures() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }

    #[test]
    fn message_passing() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hey you!");
            tx.send(val).unwrap();
        });
    }
}
