// Uncomment this block to pass the first stage
 use std::net::{TcpListener,TcpStream};
 use std::str;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    
    fn conn_handler(stream: TcpStream) {
        let mut buf = [0;4]; 
        stream.read(&mut buf);
        
        let s = match str::from_utf8(buf) {
            Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("result: {}", s);
        

    }


     let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    //
     for stream in listener.incoming() {
        match stream {
             Ok(succ_stream) => {
                 println!("accepted new connection");
                 conn_handler(succ_stream);
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}
