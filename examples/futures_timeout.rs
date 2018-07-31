extern crate futures;
extern crate futures_cpupool;
extern crate tokio_timer;

use futures::Future;
use futures_cpupool::CpuPool;
use std::time::Duration;
use tokio_timer::Timer;

fn main() {
    let pool = CpuPool::new_num_cpus();
    let timer = Timer::default();

    // a future that resolves to Err after a timeout
    let _tim = timer.sleep(Duration::from_millis(750));
    let timeout = timer.sleep(Duration::from_millis(750)).then(|_| Err(()));

    // a future that resolves to Ok with the primality result
    let prime = pool.spawn_fn(|| Ok(is_prime(BIG_PRIME)));

    // a future that resolves to one of the above values -- whichever
    // completes first!
    let winner = timeout.select(prime).map(|(win, _)| win);

    // now block until we have a winner, then print what happened
    match winner.wait() {
        Ok(true) => println!("Prime"),
        Ok(false) => println!("Not prime"),
        Err(_) => println!("Timed out"),
    }
}

const BIG_PRIME: u64 = 15485867;

// checks whether a number is prime, slowly
fn is_prime(num: u64) -> bool {
    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }
    true
}
