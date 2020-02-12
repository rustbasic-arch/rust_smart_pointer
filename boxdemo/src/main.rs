use std::ops::Add;


//
////对已经存在的类型做实现，模板的参数也是 类型的组成一部分,合成新的类型
//impl<X,Y> Add<Point<X,Y>> for Point<X,Y>{
//      type Output= Point<X,Y>;
//    fn add( self, _rhs: Point<X,Y>) -> Point<X,Y> {
//        Point::<X,Y>::new(self.x + _rhs.x, self.y + _rhs.y)
//    }
//}

trait DoDo{
    fn dodo(&self){
        println!("hello world....");
    }
}

trait Run:DoDo{// 绑架一种协议:要自己实现 要求实现另一种协议,下沉了一个特征口签名 ,拥有 DoDo的抽象特征
        fn gogo(&self);

}

//
//impl Run for DoDo{
//
//    fn gogo(&self)
//    {
//        DoDo::dodo(self);
//        println!("decorate... dodo")
//    }
//
//
//}

struct Human{

}

//Run trait 实现域
impl Run for Human{

    fn gogo(&self)
    {
//        Run::dodo(self);
        println!("gogo..impl Run for Human.  ")
    }
}
//
impl  DoDo for Human{

    fn dodo(&self){
        println!(" DoDo for Human dodo...");
    }
}


impl Human  {

    fn builders(&self){
        self.gogo();
//        Run::dodo(self);

        DoDo::dodo(self);
    }
    
}

fn recv (h :&impl DoDo)
{

    h.dodo();
}


fn main() {
let h = Human{};
//    h.builders();
    recv(&h);
//    let c = 100;
//    let ref a=100;
//    let m = &c;
//
//    let p = Point::<i32,i32>::new(10,20);
//
//    let p2 = Point::<i32,i32>::new(20,50);
//
//    let p3 = p+p2;
//
//    match p3{
//        Point{x,y}=>{
//                println!("x={},y={}",x,y);
//        }
//        _=>{
//
//        }
//
//    }
}
