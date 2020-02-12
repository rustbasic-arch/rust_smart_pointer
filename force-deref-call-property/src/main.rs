use std::ops::Deref;

struct MyWrapBox<T>{
    value:T
}

impl<T> MyWrapBox<T>{
    fn new(value:T)->Self{ //这里就是Box的实现，这里会发生move ，除非value 实现了copy
        MyWrapBox{
            value
        }
    }
}

impl<T>  Deref for  MyWrapBox<T>{
    type Target = T;
    fn deref(&self)->&T{
        &self.value
    }
}

type Name = String;

fn main() {
    let myBox = MyWrapBox{
        value:100
    };

    println!("value:{}",*myBox);

    let n :Name = "steve".to_string();

    println!("name:{}",n);

}
