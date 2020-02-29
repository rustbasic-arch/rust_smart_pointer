
trait State{

    fn handle(&self);
}


struct DefaultState(i32);
struct OnState(i32);
struct OffState(i32);


impl State for DefaultState{

    fn handle(&self)
    {
        println!("DefaultState...");
    }

}


impl State for OnState{
    fn handle(&self)
    {
        println!("OnState...");
    }

}

impl State for OffState{
    fn handle(&self)
    {
        println!("OffState...");
    }

}


enum StateValue{
    DefaultState(i32),
    OnState(i32),
    OffState(i32),
}



fn getState(i:i32)->StateValue
{
    match i {
        0=> StateValue::DefaultState(0),
        1=>
            StateValue::OnState(1),
        _=>
        StateValue::OffState(2)
        ,
    }
}

struct Context<'a>{
   s:&'a State
}

impl<'a> Context<'a>
{
  fn setState(&mut self,s:&'a State){
    self.s= s;
  }

//转移context的状态响应的代码
  fn execute(&self){
      self.s.handle();
  }
}



fn main() {

    let mut c = Context{s:&DefaultState(0) as &State};
    c.execute();

    c.setState(&OnState(1) as &State);
    c.execute();

    c.setState(&OffState(10) as &State);
    c.execute();




}
