//trait A{
//    fn a(&self){
//        println!("A trait");
//    }
//}
//
//trait B{
//    fn b(&self){
//        println!("B trait");
//    }
//}
//
//trait Foo{
//    fn f(&self){
//        println!("f trait");
//    }
//}
//
////群体A 特征的
//
//impl<T> Foo for T where T:A{
//
//    fn f(&self){
//        println!("Foo-->A trait");
//    }
//
//
//}
//
////群体B 特征的
//impl <T> Foo for T where T:B{
//    fn f(&self){
//        println!("Foo-->B trait");
//    }
//}

//针对群体A+B 特征的
//消除重复
//impl <T> Foo for T where T:A+B{
//    fn f(&self){
//        println!("Foo-->A+B trait");
//    }
//}

trait Printer{
    fn  print(&self);
}

trait Printer2{
    fn  print(&self);
}

fn Printer_recv(v:&impl Printer){

}

fn Printer2_recv(v:&impl Printer2){

}

trait NeedImplFun {
    fn recv(&self);

}

impl <T> NeedImplFun for T{
    fn recv(&self)
    {
        println!("NeedImplFun....")
    }
}

//这里T 包括：1、已知的具体类型  2、未知的 实现了 某个trait的具体类型
fn recv<T>(t: &T)
{
    t.recv();
    println!("dddd");
}

fn recv2<T:NeedImplFun>(v:&T){ //T 这里是限定 已知 的 实现了 impl NeedImplFun的 具体类型
    v.recv();
    println!("dddd222");
}



fn main() {

   let a = 100;
   a.recv();

    recv(&a);


}
