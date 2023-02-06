// Uncomment this block to pass the first stage
 use std::net::{TcpListener,TcpStream};
 use std::{str, u8};
 use std::io::{BufReader,Read,Write};
 use std::thread;
 


fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");


    fn tokenizer(bytes_buff: &mut Vec<u8>) -> Vec<u8> {
        let mut token: Vec<u8> = Vec::new();
        for i in 0 .. bytes_buff.len()-2{
            let term_slice = &bytes_buff[i..i+1];
            if term_slice == b"\r\n" {
                break;
            }
            token.push(bytes_buff[i]);
        }
        return token;
    }
    
    fn conn_handler( stream: &mut TcpStream) {
        
        
        let mut buf = [0;512]; 
        let mut reader = BufReader::new(stream);

        loop  {
        let res = reader.read(&mut buf).unwrap();
        println!("Size of msg is {}", res);     

        let s = match str::from_utf8(&buf) {
            Ok(v) => v,
       Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
       };

       let mut msg_bytes =s.as_bytes().to_vec();

       let mut token = tokenizer(&mut msg_bytes);


       println!("message: {}",s);
       println!("1st token: {}", token.len());


      }; 

    
       
    

    }


     let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    //
     for stream in listener.incoming() {
        match stream {
            
             Ok( mut succ_stream) => {
                 println!("accepted new connection");
                 thread::spawn(move || {
                 conn_handler( &mut succ_stream);
                 });
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}
