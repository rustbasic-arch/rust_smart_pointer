


trait Foo{
    fn foo(){
        println!("addr of Foo::foo:{:p}",&Self::foo);
    }

    fn print_foo_addr(&self);
}

//保存closure
struct Bar<F:Fn(i32)->i32>{
    func:F,
    i:i32,
}

impl <F> Bar<F> where F:Fn(i32)->i32{
    fn new(f:F)->Bar<F>{
        Bar{
            func:f,
            i:100
        }
    }

    fn print_new_addr(&self){
        println!("addr of Bar::new :{:p}",&Self::new);
        println!("addr of Bar::print :{:p}",&Self::print_new_addr);
    }
}

impl<F>Foo for Bar<F> where F:Fn(i32)->i32 {
    fn print_foo_addr(&self){
        Self::foo();
        println!("addr of Foo::print:{:p}",&Self::print_foo_addr);
        println!("call func value:{}",(self.func)(self.i)); //call func
    }
}

fn main(){
    let b = Bar::new(|i|{
        i+100
    });

    let b2 = Bar::new(|i|{
        i+100
    });

    b.print_foo_addr();
    b.print_new_addr();
    println!("");
    b2.print_foo_addr();
    b2.print_new_addr();
}