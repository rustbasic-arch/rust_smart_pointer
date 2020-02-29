use std::sync::Mutex;
use std::sync::Arc;


#[derive(Debug)]
struct Point{
    pub x:i32,
    pub y:i32
}

impl  Point{
    fn getInstance()-> Arc<Mutex<Point>>{
        static  mut POINT:Option<Arc<Mutex<Point>>> = None;
        unsafe {
            POINT.get_or_insert_with(||{
                Arc::new(Mutex::new(Point{x:10,y:20}))
            })
        }.clone()
    }
}


fn main() {

    let p = Point::getInstance();
    let p2 = Point::getInstance();

    println!("p1:{:p},p2:{:p}",p,p2);




}
