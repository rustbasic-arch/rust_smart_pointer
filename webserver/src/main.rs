use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, ErrorKind};
use std::fs;
use std::io::BufRead;
use std::path::Path;

//tcpConn 没有长期持有，一次性堵塞 同步
fn handleClient(mut tcpConn:TcpStream){
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
}


fn main() {
//      let paths = fs::read_dir("./").unwrap();

//      for path in paths{
//          println!("pathName:{}",path.unwrap().path().display());
//      }

    let tcpListener= TcpListener::bind("192.168.0.103:80").unwrap();
    for stream in tcpListener.incoming(){
        match stream{
            Ok(stream)=>{
                handleClient(stream);
            },
            Err(e)=>{

            }

        }
    }
}
