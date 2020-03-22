
#[derive(Clone,Debug)]
struct Task{
    id :i32
}

trait Worker{
    fn setId(&mut self,id:i32);
    fn getId(&self)->i32;
    fn recv(&mut self,task:Task);
}


struct LocalWorker{
    id:i32,
    tasks:Vec<Task>
}
struct RemoteWorker
{
id:i32,
tasks:Vec<Task>
}

impl Worker for  LocalWorker{
    fn setId(&mut self,id:i32){
        self.id =id;
    }
    fn getId(&self)->i32
    {
        return self.id;
    }
    fn recv(&mut self,task:Task)
    {
        println!("wid:{}, recv task id:{}",self.id,task.id);
        self.tasks.push(task);
    }

}

impl Worker for RemoteWorker{

    fn setId(&mut self,id:i32){
        self.id =id;
    }
    fn getId(&self)->i32
    {
        return self.id;
    }
    fn recv(&mut self,task:Task)
    {
        println!("wid:{}, recv task id:{}",self.id,task.id);
        self.tasks.push(task);
    }

}

struct WorkerContext{
    workers:Vec<Box<Worker>>  //&impl Worker
}

impl WorkerContext{
    fn new()->Self{
        WorkerContext{workers:Vec::new()}
    }
    fn addWorker(&mut self ,w:Box<Worker>){
        self.workers.push(w);
    }

    fn dispatchTask(&mut self,task:Task){

        for mut w in self.workers.iter_mut(){
            w.recv(task.clone());
        }

    }

}


fn main() {

    let mut wtx = WorkerContext::new();
    wtx.addWorker(Box::new(LocalWorker{id:1,tasks:Vec::new()}));
    wtx.addWorker(Box::new(LocalWorker{id:2,tasks:Vec::new()}));

    wtx.dispatchTask(Task{id:50});
    wtx.dispatchTask(Task{id:60});
}
