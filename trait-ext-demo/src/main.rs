

trait Name{

    fn getName(&self)->String;

}


trait Print{
    fn print(&self){
        println!("print:value ");
    }
}


impl<T:Name> Print for T{
    fn print(&self){
        println!("print name:{}",&self.getName());
    }
}

struct Teacher{

}

impl Name for Teacher{
    fn getName(&self)->String{
        "teacher ".to_string()
    }
}

fn main() {

    let t = Teacher{};

    t.print();


}
