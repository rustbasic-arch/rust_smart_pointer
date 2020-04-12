//use std::net::{TcpListener, TcpStream};
//use std::io::Write;
//
//
//
//fn handleConnection<T>(mut f:T) where  T:FnMut(){
//            f();
//}
//
//fn main() {
//   let s =  format!("{}:{}","192.168.0.103","9000");
//    let listener = TcpListener::bind(s).unwrap();
//
//    for mut conn in listener.incoming(){
//
//        match conn{
//            Ok( mut con) =>{
//
//                handleConnection(||{
//
//                    con.write("".as_bytes());
//
//
//                });
//
//            },
//            _=>{
//
//            }
//
//
//
//        }
//
//
//    }
//
//}



use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::thread::{Thread, sleep_ms};


fn handleConnection<T>(mut c:TcpStream, mut f:T) where  T:FnMut(TcpStream){
    f(c);
}

fn main() {
    let s =  format!("{}:{}","192.168.0.103","9000");
    let listener = TcpListener::bind(s).unwrap();

    for mut conn in listener.incoming(){

        match conn{
            Ok( mut con) =>{

                handleConnection(con,|mut c:TcpStream|{

                    loop {
                        c.write("helloworld,,,,,".as_bytes());
                        sleep_ms(1000);

                    }



                });

            },
            _=>{

            }



        }


    }

}
