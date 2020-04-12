
trait Implementable{
    fn load(&self);
}


struct AImpl {

}

impl  Implementable for AImpl{
    fn load(&self)
    {
        println!("AImpl load ....")
    }
}
struct BImpl{

}

impl Implementable for BImpl{
    fn load(&self)
    {
        println!("BImpl load ....")
    }
}

struct Context<'a>{
     base:&'a  Implementable
}

impl<'a> Context<'a>{

    fn load(&self){
        self.base.load();
    }

    fn setImpl(&mut self,imp:&'a Implementable)
    {
        self.base = imp;
    }
}






fn main() {
    let mut c = Context{base:& AImpl{} };
    c.load();
    c.setImpl(&BImpl{});
    c.load();
}
