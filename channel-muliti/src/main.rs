use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

fn main() {
    // 在多个线程之间使用的临界数据
    let data = Arc::new(Mutex::new(0u32));
    println!("{:?}", data);

    // 对多线程同步的通道
    let (tx, rx) = mpsc::channel();

    // 启动10个子线程修改临界区数据
    //in sub thread
    for _ in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());

        thread::spawn(move || {
            // 加锁
            let mut data = data.lock().unwrap();
            // 修改临界区数据
            *data += 1;
            // 解锁 => 给通道发送消息，解除阻塞，执行下一个任务
            tx.send(());
            println!("data => {:?}", *data);
        });
    }

    //block in main thread
    // 控制10个子线程按照顺序依次执行
    for _ in 0..10 {
        rx.recv();
    }

    println!("{:?}", data);
}