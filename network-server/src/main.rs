
use std::thread;
use std::sync::Mutex;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;



fn add(c:Arc<Mutex<i32>>){
    let mut numPtr =   c.lock().unwrap();
    *numPtr=*numPtr+100;
    println!("value:{}",*numPtr);

}

fn main() {
    pub fn run_unsleep() {
        let worker =4;

        let count =Arc::new(Mutex::new(0));

        let mut handlers = vec![];
        for j in 0..4{
            let cp =count.clone();
            let h = thread::spawn( || {
                    add(cp);
                    thread::sleep(std::time::Duration::from_secs(1));
            });
            handlers.push(h);
        }

        for h in handlers {
            h.join().unwrap();
        }
    }
    run_unsleep();

}
