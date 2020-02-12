use std::cell::RefCell;

trait A{

}

impl A for u8{

}

impl A for u16{

}



fn func(x: &A) {
}

fn func_mut(x: &RefCell<A>)  {

      *x.borrow_mut() = 1000;




}

fn main() {
    let  num = RefCell::new(100u16);

    func_mut(&num);
}