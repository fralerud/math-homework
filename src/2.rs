use std::{thread, time};

fn main() {
    let sleep_time = 1;

    // Sleep for a random amount of time
    thread::sleep(time::Duration::from_secs(rand::random::<u64>() % sleep_time));
}
