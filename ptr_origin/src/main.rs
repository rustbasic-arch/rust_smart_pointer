


fn main() {

    let mut x = 5;
    let raw = &mut x as *mut i32;

    let points_at = unsafe { *raw =100; *raw};

    println!("raw points at {}", points_at);

    let y = 200;
    let raw2 = &y as *const i32;

    let z= unsafe {*raw2};

    println!("y:{}",z);

}
