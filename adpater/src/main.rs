
trait TargetInterface{
    fn doTarget(&self);
}



trait NowInterface{
    fn doNow(&self);
}


struct TargetContext<'a>{
      imp:&'a TargetInterface
}

struct NowImpl{

}

impl NowInterface for NowImpl{
    fn doNow(&self)
    {
        println!("已有接口....doNow ")
    }
}


impl TargetInterface for NowImpl{

    fn doTarget(&self)
    {
        self.doNow();
        println!("已有接口....doNow  在 目标接口上面运行。。。。 ")
    }
}


impl<'a> TargetContext<'a>
{
    fn execute(&self){
        self.imp.doTarget();
    }
}



fn main() {
     let c = TargetContext{imp:&NowImpl{} as &TargetInterface};
         c.execute();

}
