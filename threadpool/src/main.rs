
use std::thread::JoinHandle;
use std::thread;
use std::sync::Mutex;
use std::sync::Arc;
use std::time::Duration;

pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

pub type Task = Box<FnBox + Send>;
struct  Worker{
    handler:Option<JoinHandle<()>>,
    wid:usize,
    tasks:Arc<Mutex<Vec<Box<Task>>>>
}

impl Worker{
    fn new(i:usize)->Worker{
        return Worker{handler:None,wid:i,tasks:Arc::new(Mutex::new(Vec::new()))};
    }

    fn start(&mut self){
        let  taskList = self.tasks.clone();
        let handler = thread::spawn(|| {
            let mut vlist=   taskList.lock().unwrap();
            loop {
                let mut task = vlist.pop();
                // task.unwrap().process();
                thread::sleep(Duration::from_secs(2));
            }
        });
        self.handler = Some(handler);
    }

    fn add(&mut self,t:Box<Task>)
    {
        let s = self.tasks.lock().unwrap();
        // println!("{}",s);
    }
}

struct WorkerConext{
    workers:Vec<Worker>
}

impl   WorkerConext{
    fn new(s:usize)->WorkerConext{
        assert!(s>0);
        let mut workers = vec![];
        for i in 0..s{
            workers.push(Worker::new(i))
        }
        WorkerConext{workers:workers}
    }

    fn push(&mut self,t:Box<Task>){
        let i =  t.getId();
        let m =  i% (self.workers.len()) ;
        self.workers[m].add(t);
    }
}

impl Drop for WorkerConext{
    fn  drop(&mut self){
        for w in &mut self.workers{
            if let Some(h) = w.handler.take(){
                h.join().unwrap();
            }
        }
    }
}


struct Request{
    id:usize
}



fn main() {
    let mut w  = WorkerConext::new(3);
    for i in 0..100  {
        w.push(Box::new(Request{id:i}));
    }

}
