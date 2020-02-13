use std::cell::{RefCell, Ref};



//思考 trait Object 的设计方式;
//关注点
//1 外部是不是需要分享
//2 是不是要关注并发访问
//3 外部需不需要保持 所有权



//本例：会消除外部所有权
trait Sprite{
    fn update(&mut self);
}


struct Monster{

}
struct Hero{

}

impl Sprite for Monster{

    fn update(&mut self){
        println!("Monster render ");
    }
}


impl Sprite for Hero{

    fn update(&mut self)
    {
        println!("Hero render ");
    }

}

struct GameScene{
    //摄入 内聚性：一般是 RefCell<Sprite> ,意味着 会 消除外部的所有权，直接 move到 本RefCell 里面，如果需要分享特征，可以 RefCell<Arc<T>> 或者 RefCell<Rc<T>>
    renderObjects:Vec<Box<RefCell<Sprite>>> ///RefCell 内部 调成了内部可以变 borrow_mut()->RefMut<T> ,如果 此时 Sprite 需要被外部分享，那么可以加一层 Arc(Rc) 保证可以分享，同时不内存泄漏
}

impl GameScene{

}

fn renderAll(g:&mut GameScene){
    for e in g.renderObjects.iter(){
        let s = (**e).borrow_mut();
          (**e).borrow_mut().update();//有智能指针的阻隔的话，访问节奏放缓一点;没有的阻隔的话，就有自动解引用透明穿透
    }
}

fn main() {

    let mut g = GameScene{
        renderObjects:vec![]
    };

    let monster = Monster{};
    let hero = Hero{};

    g.renderObjects.push(Box::new(RefCell::new(monster)));
    g.renderObjects.push(Box::new(RefCell::new(hero)));

    renderAll(&mut g);

}
