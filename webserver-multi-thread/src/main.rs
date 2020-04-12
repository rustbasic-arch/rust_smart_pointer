use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::thread::JoinHandle;
use std::fs;
use std::io::Read;

const IP:&'static str = "192.168.0.103";
const PORT:i32 = 80;


struct ClientHandler{
    conn:Option<TcpStream>
}

fn write(tcpConn:&mut TcpStream){
    let mut buffer = [0;512];
    tcpConn.read(&mut buffer).unwrap();//unwrap();move ownership

    let GET_REQUEST = b"GET / HTTP/1.1\r\n";

    let (statusLine,fileName)= if buffer.starts_with(GET_REQUEST){
        ("HTTP/1.1 200 OK\r\n\r\n", "./main.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "./404.html")
    };

    println!("fileNAME:{}",fileName.clone());
    let contentText = fs::read_to_string(fileName);

    match contentText {
        Ok(s)=>{
            let response = format!("{}{}",statusLine,s.as_str());
            tcpConn.write(response.as_bytes());
            tcpConn.flush().unwrap();
        },
        Err(e)=>{
            println!("error:{:?}",e);
        }
    }

    println!("thread tick ......")
}

impl ClientHandler{

    fn new(conn:TcpStream)->ClientHandler{
        ClientHandler{conn:Some(conn)}
    }

    fn start(&mut self)->JoinHandle<()>{
        let mut connM =self.conn.take();
        std::thread::spawn(move ||{
            if let Some(mut connn) = connM{
                loop{
                    thread::sleep(Duration::from_secs(1));
                    write(&mut connn);
                }
            }

        })//BLOCK MAIN thread block ,主线程会堵塞，所以，这里 子线程不需要 join，join函数一调用会堵塞主线程，放弃cpu,join的目的让主线程等待，子线程有机会调度
    }
}



fn main() {

    let files = fs::read_dir("./");

    for file in files.unwrap(){

        println!("fileName:{}",file.unwrap().path().display());

    }

//   let address = format!("{}:{}",IP,PORT);
//   let listener = TcpListener::bind(address).unwrap();
//
//    for conn in listener.incoming(){//BLOCK MAIN thread block ,主线程会堵塞，所以，子线程不需要 join
//        ClientHandler::new(conn.unwrap()).start();
//    }
}
