// Uncomment this block to pass the first stage
 use std::net::{TcpListener,TcpStream};
 use std::str;
 use std::io::{BufReader,Read,Write};
 

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    
    fn conn_handler(mut stream:  &TcpStream) {
        
        
        let mut buf = [0;512]; 
        let mut reader = BufReader::new(stream);

        loop  {
        let res = reader.read(&mut buf).unwrap();
        println!("Size of msg is {}", res);     
        
        

        

    //     let s = match str::from_utf8(&buf) {
    //         Ok(v) => v,
    //    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    //    };

       let simple_resp = b"+PONG\r\n";
       let bytes_written =stream.write(simple_resp);

      }; 

    
       
    

    }


     let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    //
     for stream in listener.incoming() {
        match stream {
             Ok( succ_stream) => {
                 println!("accepted new connection");
                 conn_handler( &succ_stream);
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}
