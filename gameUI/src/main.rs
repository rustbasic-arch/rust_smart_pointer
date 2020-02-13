extern crate core;

use std::rc::Rc;
use std::cell::RefCell;
use core::borrow::BorrowMut;


trait Sprite{
    fn render(& mut self);
}

trait GameSceneRender{

    fn renderScene(&self);
}

struct Game<T>{
    objects:Vec<Box<RefCell<T>>>
}

impl<T:Sprite> Game<T> {
    fn addM(&mut self,s:Box<T>)
    {
        self.objects.push(Box::new(RefCell::new(*s)));
    }

    fn addH(&mut self,s:Box<T>)
    {
        self.objects.push(Box::new(RefCell::new(*s)));
    }
    fn renderAll(&self) {
        let it = self.objects.iter();

        for ele in it {
             let mut s =(**ele).borrow_mut();
             s.render();
        }
    }
}

impl<T:Sprite> GameSceneRender for Game<T>{
    fn renderScene(&self){
        self.renderAll();
    }
}


struct Hero{
    score:i32
}

struct Monster{
    score:i32
}

impl Sprite for Hero{
    fn render(&mut self)
    {
        self.score =110;
        println!("Hero score={}",self.score);
    }
}

impl Sprite for Monster{
    fn render(&mut self){
        self.score =100;
        println!("Monster score={}",self.score);
    }
}


fn doRender(r:& impl GameSceneRender)
{
    r.renderScene();
}


struct GameMgr{
    renders:Vec<Box<GameSceneRender>>
}

impl GameMgr{
//    fn add222(&mut self,r:GameSceneRender)  trait 无法裸露的传递
//    {
//    }
    fn add(&mut self,r:Box<GameSceneRender>) //move
    {
        self.renders.push(r);
    }
}

impl GameMgr
{
    fn foreachRender(&self){
        for sceneRender in self.renders.iter(){
            sceneRender.renderScene();
        }
    }
}


fn main() {

    //由于采用了模板， Monster
    let mut monsterGame = Game{
        objects:vec![]
    };
    let m = Monster{score:100};
//    game.add(Box::new(m));  //注释报错，不注释 可以编译通过 ,结论：要嘛 终端使用不要引入 Game ,否则 一旦引入就要，类型可以推断 出 类型T

    monsterGame.addM(Box::new(m));//这样 编译器通过 Box::new 的参数类型 Monster ，可以推断   T 就是Monster



    let mut herogame = Game{
        objects:vec![]
    };
//
    let h = Hero{score:10000};
    herogame.addH(Box::new(h));  //这里报错，由于采用了模板trait,是的


   let mut mgr = GameMgr{renders:vec![]};

    mgr.add(Box::new(monsterGame));
    mgr.add(Box::new(herogame));

    mgr.foreachRender();


}
