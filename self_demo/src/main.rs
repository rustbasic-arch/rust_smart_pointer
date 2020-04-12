
//
//struct Msg{
//    content:&'static str,
//    msgType:i32
//}
//
//
//impl Msg{
//    fn send(mut self,content:String) //content 是外部传入的，转化为指针，接受者 content:&'static str，不能确定传入的 String::new().as_str() 的生命周期
//    {
//        self.content=content.as_str();
//    }
//}
//
//
//fn main() {
//
//    let m = Msg{content:"",msgType:12};
//    m.send("hellworld".to_string());
//
//}


//struct Msg{
//    content:&'static str,
//    msgType:i32
//}
//
//
//impl Msg{
//    fn send(&mut self,content:&'static str) // 统一采用 'static str ;总结：input :&'staitc str ,内部接受 &str
//    {
//        self.content=content;
//    }
//    fn printMsg(&mut self){
//            println!("content:{}",self.content);
//    }
//}
//
//fn main() {
//    let mut m = Msg{content:"",msgType:12};
//    m.send("hellworld");
//
//    m.printMsg();
//}


struct Msg{
    content:String,
    msgType:i32
}



trait Proto{
    fn getBytes(&self)->Vec<u8>;
}
trait Pusher{
    fn push(&self,proto:impl Proto);
}

impl Pusher for Msg{
    fn push(&self,proto:impl Proto){
        //vec<u8> 字节数组转为 字符串
        let b = proto.getBytes();
        let bytesVec2String = String::from_utf8(b.to_owned()).unwrap();
        println!("bytesVec2String:{}",bytesVec2String.as_str());
    }
}

impl Msg{


}

struct MsgProto{

}


impl Proto for MsgProto
{
    fn getBytes(&self)->Vec<u8>{
        let mut  v = vec![65_u8,66_u8,97];
            v
    }
}
fn main(){
        let msg =Msg{content:"hellworld".to_string(),msgType:10};
        msg.push(MsgProto{});

    let ss = "abc";
    ss.to_string();

    let bb = ss.to_string()+"hellworld";

    let b= &[97_u8,98,99];
    let b1 =&b[0..1];
    let mut b2 =&b[0..1];

    let s =  String::from_utf8_lossy( b1);
    let s2 = String::from_utf8_lossy(b2);


    println!("s1={},s2={}",s,s2);
}