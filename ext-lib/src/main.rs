

trait ExtFun{
    fn execute(&self);
}


impl<T:FnOnce()>  ExtFun for T{
    fn execute(&self)
    {
        println!("ext fn ....");
        (*self)();
    }

}


pub type Task = Box<ExtFun+Send>;



fn main() {






}
