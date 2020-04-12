
trait Student{
    fn new()->Self;
}

struct S{

}

impl Student for S{
    fn new()->Self
    {
        S{}
    }

}
impl  S{
    fn getName(self:&S)->String{
        return "hello".to_string();
    }
    fn getName2(self:&S)->String{
        return "hello".to_string();
    }
}

fn callFun<T>(input:i32,f:T)->i32 where T:Fn(i32)->i32{
       f(input)
}


fn main() {

    let  s =Box::new(S::new());
    //s->Box<S> 调用方法  getName(self:Box<S>)，box是对象传递，会move进入函数
    print!("mmm:{}",s.getName());//所以再次使用S 不可以

    s.getName2();

    let mut c =100;
    let result= callFun(c,|i:i32|->i32{
                    i+100000
    });

    println!("result:{}",result);

}
