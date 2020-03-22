
/**
*observer from wwp
*/

#[derive(Clone,Debug)]
struct Task{
    tid:i32
}


//struct TaskRouter{
//    tasks:Vec<Box<Task>>
//}

struct Worker{
    id :i32,
    tasks:Vec<Box<Task>>
}

impl Worker{
    fn recv(&mut self,task:Box<Task>){
        println!("worker id:{}, recv taskid:{}",self.id,task.tid);
        self.tasks.push(task);
    }

    fn new(id:i32)->Self{
        Worker{id:id,tasks:Vec::new()}
    }
}

struct WorkerContext{
    workers:Vec<Box<Worker>>

}


impl WorkerContext{

    fn dispatch(&mut self,task:Box<Task>){
        for  w in self.workers.iter_mut(){
            w.recv(task.clone());
        }
    }

    fn registerWorker(&mut self,w:Box<Worker>){
        self.workers.push(w);
    }

    fn new()->Self{
        WorkerContext{workers:Vec::new()}
    }
}


fn main() {
    let mut workCtx = WorkerContext::new();

    workCtx.registerWorker(Box::new(Worker::new(1)));
    workCtx.registerWorker(Box::new(Worker::new(2)));
    workCtx.registerWorker(Box::new(Worker::new(3)));

    let btask = Box::new(Task{tid:100});

    workCtx.dispatch(btask);

}
