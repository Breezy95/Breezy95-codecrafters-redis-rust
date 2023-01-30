// Uncomment this block to pass the first stage
 use std::net::{TcpListener,TcpStream};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    
    fn conn_handler(stream: TcpStream) {
        let mut buf = [0;10]; 
        let len = stream.peek(&mut buf).expect("peek op failed");
        println!("value of the length of message is {}", len);
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
