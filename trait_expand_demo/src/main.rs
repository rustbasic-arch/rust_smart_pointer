// 接口定义
trait Graph {
    type N;
    type E;

    fn render(&self, &Self::N, &Self::N) -> bool;
//    fn yes(&self, &Self::N) -> Vec<Self::E>;
}

// 全局函数，函数形参的类型为接口类型
fn distance<G: Graph>(
    graph: &G,
    start: &G::N,
    end: &G::N) -> i32 {

    return 99;
}

struct GameObject{

}

impl Graph for GameObject{
    type N = i32;
    type E =i32;
    fn render(&self, n :&Graph::N, m: &Graph::M) -> bool
    {
//            println!("n ={},m={}");
    }
}


fn main()
{

    let g = GameObject{};

//    distance(g);

    g.render()
}