extern crate core;

use std::rc::Rc;
use std::cell::{RefCell, Ref};
use core::borrow::{BorrowMut, Borrow};


struct  Node<T>{
    pre:Option<Rc<RefCell<Node<T>>>>,
    next:Option<Rc<RefCell<Node<T>>>>,
    value :T
}

impl<T> Node<T>{
        fn new(v:T)->Option<Rc<RefCell<Node<T>>>>{
           Some(Rc::new(RefCell::new( Node{
               pre:None,
               next:None,
               value:v
           })))
        }

    fn print(&self){
        println!("dddd");
    }
}



fn main() {
    let mut n = Node::new(100);
    match n {
        Some(  f)=>{

           let mut m=  (*f).borrow_mut();

            m.next = Node::new(100);

        },
        _=>{

        }
    }






    println!("Hello, world!");
}
