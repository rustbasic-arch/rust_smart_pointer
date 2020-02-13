
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

fn recvBoxTrait(r:Box<Runnable>){
    r.run();
}


fn recvImplType(r:&impl Runnable){
    r.run();
}


fn main() {

    let h = Human{};
    recv(&h);

    recvImplType(&h);
    recvBoxTrait(Box::new(h));


    let p = Person{};

    recv(&p);
    recvImplType(&p);

    recvBoxTrait(Box::new(p));

}
