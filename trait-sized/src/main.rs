
//被输入的 参数的地方要加 被输入方要加 生命周期参数
enum MyResult<'a> {
    Good(&'a GoodReponser), //dyn trait --->调用方 一定要 Sized trait 的Self
    Bad(&'a  BadReponser), //dyn trait  ---->调用方 一定要确定 BadReponser 可以确定 Sized 的Self
}

trait BaseResponser {
    fn onResult(&self) -> MyResult;
    fn action(self);
}

//specific trait # 1 更具体一点的特征
trait GoodReponser: BaseResponser {
    fn onResult_Inner(&self) -> MyResult /*where Self: Sized*/
    {
        MyResult::Good(self)
    }

    fn saveWorld(&self);
}

//specific trait #2 更具体一点的特征
trait BadReponser: BaseResponser {
    fn onResult_Inner(&self) -> MyResult where Self:Sized {
        MyResult::Bad(self)
    }

    fn doEvil(&self);
}

//impl trait 编译时，可以从上下文获得 具体类型
//dyn  trait 在编译时没法获得，动态才知道它的类型大小


fn main() {
    println!("Hello, world!");
}
