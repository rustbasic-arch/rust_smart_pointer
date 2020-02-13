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

struct Follower{

}


impl Sprite for Follower{
    fn update(&mut self){
        println!("Follower render ");
    }

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
    renderObjects:Vec<Box<RefCell<Sprite>>> //RefCell 内部 调成了内部可以变 borrow_mut()->RefMut<T> ,如果 此时 Sprite 需要被外部分享，那么可以加一层 Arc(Rc) 保证可以分享，同时不内存泄漏
}

impl GameScene{

    fn render(&mut self)
    {
        for e in self.renderObjects.iter(){
            let mut s = (**e).borrow_mut();
            s.update();//有智能指针的阻隔的话，访问节奏放缓一点;没有的阻隔的话，就有自动解引用透明穿透
        }
    }

    fn addObject(&mut self,o:Box<RefCell<Sprite>>){
        self.renderObjects.push(o);
    }

    fn addObjectByTemplate<T:Sprite+'static >(&mut self,o:T){
        self.renderObjects.push(Box::new(RefCell::new(o)));
    }

}

fn renderAll(g:&mut GameScene){
    for e in g.renderObjects.iter(){
        let mut s = (**e).borrow_mut();
        s.update();//有智能指针的阻隔的话，访问节奏放缓一点;没有的阻隔的话，就有自动解引用透明穿透
    }
}

fn main() {

    let mut g = GameScene{
        renderObjects:vec![]
    };

    let monster = Monster{};
    let hero = Hero{};
    let follower = Follower{};

    g.addObject(Box::new(RefCell::new(monster)));//RefCell 可以将一个不可变的对象，转成可以变
    g.addObject(Box::new(RefCell::new(hero)));
//    g.addObject(Box::new(RefCell::new(follower)));


    //采用方法模块统一接口，实际本质是生成了 “类型一致性” 类型拼接好的唯一方法 :好比addObject_Follower()
    g.addObjectByTemplate(follower);
    //总结：接口简洁之道： 1、采用方法模板 template+ trait ：方法addObjectByTemplate说明：这种编程风格,模板 trait 特化，生成的代码增加，但是用户使用的API简洁;
    //2、仅仅box+trait ,方法addObject 对外公开接口简洁，但是用户写的代码较多


    g.render();
    renderAll(&mut g);
}
