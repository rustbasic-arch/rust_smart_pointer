trait Man {
    fn name(&self) -> String;
}

// 接口2
trait Animal {
    fn name(&self) -> String
    {
        return "AAAAAAAAAAAAAA".to_string();
    }
}

// 接口3
trait Runnable {
    fn run(&self);
}



struct S_Man{

}

impl S_Man {
    fn say(&self){
        println!("aniname:{},Man::name:{}",&Animal::name(self), &Man::name(self)) ;
    }

}

impl Runnable for S_Man{

    fn run(&self)
    {
        println!("Runnable for S_Man run ....");
    }

}

impl Animal for S_Man{

    fn name(&self) -> String{
        return "ssssssssssssss-name".to_string();
    }

}

impl Man for S_Man{
    fn name(&self) -> String{
        "hahja".to_string()
    }
}



fn main()
{

    let s = S_Man{};
    s.say();





}