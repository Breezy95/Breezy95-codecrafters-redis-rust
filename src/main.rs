// Uncomment this block to pass the first stage
 use std::net::{TcpListener,TcpStream};
 use std::{str, u8};
 use std::io::{BufReader,Read,Write, BufRead};
 use std::thread;
 


 


fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");


    fn tokenizer(bytes_buff: &mut Vec<u8>) -> Vec<String> {
        let mut tokens: Vec<String> =  Vec::new();
        
        let buffer_slice = &bytes_buff[..];
        let mut reader = BufReader::new(buffer_slice);
        //let mut buf: Vec<u8> = vec![];
        for line in reader.lines(){
            tokens.push(line.unwrap())
            println!("line: {}",line.unwrap());
        }
        

        return tokens; 
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

       let mut msg_bytes = buf.to_vec();
       
       let mut cmd_stk: Vec<u8> = Vec::new();

       let mut tokens = tokenizer(&mut msg_bytes);

       


       //println!("message: {}",s);
       println!("1st token: {}", tokens.len());


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
