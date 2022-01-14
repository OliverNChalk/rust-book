use rand;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let counter1 = Arc::new(Mutex::new(1));
    let counter2 = Arc::new(Mutex::new(1));

    let mut handles = vec![];
    for _ in 0..2 {
        let counter1 = Arc::clone(&counter1);
        let counter2 = Arc::clone(&counter2);

        let handle = thread::spawn(move || loop {
            let mut count1;
            let mut count2;
            if rand::random::<u32>() % 100 == 0 {
                // 1 in 100 chance we take lock on counter1 first
                count1 = counter1.lock().unwrap();
                thread::sleep(Duration::from_micros(500));
                count2 = counter2.lock().unwrap();
            } else {
                count2 = counter2.lock().unwrap();
                thread::sleep(Duration::from_micros(500));
                count1 = counter1.lock().unwrap();
            }

            *count1 += 1;
            *count2 += 1;

            let mean_count = *count1 + *count2 / 2;

            println!("mean count {}", mean_count);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
