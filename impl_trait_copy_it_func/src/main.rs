
trait Foo{
    fn foo(){
        println!("foo");
    }
}

struct Bar1;
struct Bar2;

impl Foo for Bar1{}
impl Foo for Bar2{}

fn main(){
    println!("{:p}",&Bar1::foo);
    println!("{:p}",&Bar2::foo);
}