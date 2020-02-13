use std::fs::DirBuilder;

trait Runnable{
    fn run(&self);
}

struct Human{

}

impl Runnable for Human{

    fn run(&self)
    {
        println!("Human running ");
    }

}




struct Person{

}

impl Runnable for Person {
    fn run(&self)
    {
        println!("Person running ");
    }
}

fn recv(r:&Runnable){
        r.run();
}

fn recvDirectPass<T:Runnable>(r:T){
    r.run();
}

fn recvBoxTrait(r:Box<Runnable>){
    r.run();
}

//模板化 Trait 可以适用 终端具体类型
fn recvBoxTrait_0<T:Runnable>(r:Box<T>){
    r.run();
}


fn recvImplType(r:&impl Runnable){
    r.run();
}


fn recvBoxTrait2(r:&Box<Runnable>){
    r.run();
}

fn recvBoxTrait3<T:Runnable>(r:&Box<T>){ //解决了 Box<ImplType> ===》 Box<Trait> 的问题
    r.run();
}

fn main() {

    let h0 = Human{};
    recvDirectPass(h0);



//第一快 方法
    let h = Human{};


    recv(&h);
    recv(&h);

    recvImplType(&h);



    let p = Person{};

    recv(&p);
    recv(&p);
    recvImplType(&p);

    //第二快 方法  move
    recvBoxTrait(Box::new(p));
    recvBoxTrait(Box::new(h));

  //方法三

//    let humanBox = Box::new(Human{});//错误 无法将  (输入方) Box<Human> ===> (接受方) Box<Trait> 类型，通过 模板化 trait ，适应终端具体子类型
//    recvBoxTrait2(&humanBox);


   //方法四

    let humanBox = Box::new(Human{});//错误 无法将 Box<Human> ===> Box<Trait> 类型
    recvBoxTrait3(&humanBox);//第一次调用
    recvBoxTrait3(&humanBox);//第二次调用
}
