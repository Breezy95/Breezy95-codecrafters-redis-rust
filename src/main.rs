// Uncomment this block to pass the first stage
 use std::net::{TcpListener,TcpStream};
 use std::{str, u8, vec};
 use std::io::{BufReader,Read,Write, BufRead};
 use std::thread;
 


 


fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");


    fn tokenizer(bytes_buff: &mut Vec<u8>) -> Vec<String> {
        let mut tokens: Vec<String> =  Vec::new();
        
        let buffer_slice = &bytes_buff[..];
        let reader = BufReader::new(buffer_slice);
        //let mut buf: Vec<u8> = vec![];
        for line in reader.lines(){
            let elem = line.as_ref();
            tokens.push(elem.unwrap().to_string());
            println!("line: {}",elem.unwrap());
        }
        

        return tokens; 
    }

    
    
    fn conn_handler( stream: &mut TcpStream) {
        
        
        let mut buf = [0;512]; 
        let mut reader = BufReader::new(stream.try_clone().unwrap());

        loop  {
        let res = reader.read(&mut buf).unwrap();
        println!("Size of msg is {}", res);     

    //     let s = match str::from_utf8(&buf) {
    //         Ok(v) => v,
    //    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    //    };

       let mut msg_bytes = buf.to_vec();
       
       let tokens = tokenizer(&mut msg_bytes);
       let  (mut arrs,mut BStrs,mut Integers,mut Errors,mut SStrs) = (0,0,0,0,0);
       let mut op_vec: Vec<String> = vec![];
       for token in tokens{
        let mut iter =token.chars();
        let first_char = iter.nth(0);
        iter.next();

        let subseq: String = iter.collect();
        println!("first_char: {},subseq chars: {}",first_char.unwrap(),subseq);
       match first_char.unwrap() {
        '+' => SStrs+=1,
        '-' => Errors+=1,
        ':' => Integers+=1,
        '$' => BStrs +=1,
        '*' =>  op_vec.push("_".to_string()),
        //all chars
        _ => {}
       }

    }
       

       let ans =stream.write(b"PONG");
       


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
