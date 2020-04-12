

trait ICommand{
    fn exe(&self);
    fn hz(&self);
    fn sec(&self)->i32;
}

impl ICommand for i32{
    fn exe(&self)
    {
        print!("expand i32 by command");
    }

    fn hz(&self)
    {

    }
    fn sec(&self)->i32
    {
        return *self+100;
    }

}

fn main() {
    let a = 10;
    a.exe();
    let res = a.sec();
    println!("res={}",res);

}
