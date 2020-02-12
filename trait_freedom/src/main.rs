
trait Connection{
    fn connect(&self,ip:String,port:i32)
    {
        println!("connection ip:{},port :{}",ip,port);
    }


}

trait Client:Connection{

    fn send(&self,msg:String);

}

struct Model_Client{


}

impl Model_Client {

}


impl Client for Model_Client{
    fn send(&self,msg:String)
    {
        println!("Model_Client send msg:{}",&msg);
    }
}



impl Model_Client{
    fn new()->Self{
        Model_Client{

        }
    }
}

fn exec(conn :&impl Connection ,ip:String,port:i32){
    conn.connect(ip.to_owned(),port);
}

fn main() {

    let m = Model_Client::new();
    exec(&m,"127.0.0.1".to_string(),9000);



}
