
use std::sync::Mutex;
use std::sync::Arc;
struct Singleton2{

}

impl Singleton2{
    fn getInstance()->Arc<Mutex<Singleton2>>{//欢迎
        static mut s :Option<Arc<Mutex<Singleton2>>> = None;
        unsafe {
            s.get_or_insert_with(|| {
                Arc::new(Mutex::new(Singleton2 {}))
            }).clone()
        }
    }
}


struct Singleton3{

}

impl Singleton3{

    fn getInstance()->Arc<Mutex<Singleton3>>{
        static mut ins :Option<Arc<Mutex<Singleton3>>> = None;
        unsafe {
            ins.get_or_insert(Arc::new(Mutex::new(Singleton3{})))
        }.clone()
    }

}

fn main() {


    let s = Singleton2::getInstance();

    let s2 = Singleton2::getInstance();

    println!("s1:{:p},s2={:p}",s,s2);


    let s33 = Singleton3::getInstance();

    let s233 = Singleton3::getInstance();


    println!("s33:{:p},s233={:p}",s33,s233);
}
