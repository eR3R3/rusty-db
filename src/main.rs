use std::thread;
use std::time::Duration;
use db::pool::ThreadPool;


fn main() {
    let pool = ThreadPool::new(8);
    for i in 0..9 {
        pool.execute(move || {
            thread::sleep(Duration::from_secs(4));
            println!("{}", i)
        });
    }
    thread::sleep(Duration::from_secs(9));
}





