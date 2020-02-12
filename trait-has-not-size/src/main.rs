
trait A{

}

impl A for u8{

}

impl A for u16{

}



fn func(x: &A) {
}

fn func_mut(x: &mut A)  {
//   let z = x as *const i16;



}

fn main() {
    func(&0u8);
    func_mut(&mut 0u16);
}