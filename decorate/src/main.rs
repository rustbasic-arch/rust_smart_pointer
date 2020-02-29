
trait Component{
    fn read(&self);
}

//业务实现
trait BaseObject:Component{
    fn read(&self);
}


struct ATarget{

}

struct Decorate<'a>{
    base :&'a Component,
}
impl BaseObject for ATarget{
    fn read(&self)
    {

    }
}

impl Component for ATarget{
    fn read(&self)
    {
        println!("ATarget read... ");
    }
}

impl<'a> Component for  Decorate<'a>{
    fn read(&self){
       self.base.read();
        println!("Decorate read... ");
    }
}


struct DecorateBufferReader<'a>{
    tar:&'a Component
}

impl<'a> Component for DecorateBufferReader<'a> {
    fn read(&self){
        self.tar.read();
        println!("Decorate DecorateBufferReader ... ");
    }

}

fn main() {
   let target = ATarget{};

    let dec = Decorate{base:&target as &Component};
    dec.read();

    let reader = DecorateBufferReader{tar:&dec as &Component};

    reader.read();
}
