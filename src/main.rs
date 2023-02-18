use std::iter::Peekable;
// Uncomment this block to pass the first stage
 use std::net::{TcpListener,TcpStream};
 use std::slice::Iter;
use std::{str, u8, i32};
 use std::io::{BufReader,Read,Write, BufRead};
 use std::thread;
 use std::collections::HashMap;

 
 fn encode() {

 }

fn decode() {

}



fn set_values(mut kvmap: HashMap<String, String>, kv :&mut Peekable<Iter<String>>) -> Result<Option<String>, &'static str>{
    
    let values = kv.clone();
    if values.len() < 2 {
        return Err("no valid key");
    }
    let  key = kv.next().unwrap().to_owned();
    let val = kv.next().unwrap().to_string().to_owned();
    
    let old_value =kvmap.insert( key, val.to_owned());


    return Ok(old_value);  
}

fn get_values(key: String, kvmap: HashMap<String, String>) -> Result<String, &'static str> {
let value = kvmap.get(&key);
    if value.is_none(){
        return Err("value is not in map");
    }
    return Ok(value.unwrap().to_string());


}
 


fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");


    fn tokenizer(bytes_buff: &mut Vec<u8>) -> Vec<String> {
        let mut tokens: Vec<String> =  Vec::new();
        
        let buffer_slice = &bytes_buff[..];
        let reader = BufReader::new(buffer_slice);
        //let mut buf: Vec<u8> = vec![];
        for line in reader.lines(){
            if line.as_ref().is_err(){
                break
            }
            let elem = line.as_ref();
            tokens.push(elem.unwrap().to_owned());

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
      
       let mut op_vec: Vec<String> = Vec::new();
       //let mut operands: Vec<String> = Vec::new();

       
       for token in &tokens[..]{
        let mut iter =token.chars();
        let first_char = iter.nth(0);

        let subseq: String = iter.collect();
        //println!("first_char: {},subseq chars: {}",first_char.unwrap(),subseq);


       match first_char.unwrap() {
        '$' => {},
        //array
        '*' =>  op_vec =  Vec::with_capacity(str::parse(&subseq[..]).unwrap()),
        //all chars
        _ => {if op_vec.len()<= op_vec.capacity() {op_vec.push(token.to_string());};
              
        }
       }
    }
    //use iter instead of indexing
    let mut op_iter = op_vec.iter().peekable();
    let operation: &str =op_iter.peek().unwrap();
    let mut kvpairs: HashMap<String, String> = HashMap::new();
    println!("operation: {}", operation);
    match operation {
        "ping"  => {let len =stream.write(b"+PONG\r\n");
        println!("Sent payload of len: {}", len.unwrap());
    },

        "echo" => {let byte_str = b"+";
                   let packet =[byte_str, op_vec[1].as_bytes(), b"\r\n"].concat();        
                   stream.write(&packet[..]);
                  },

        "set" => { 
                let mut iter_clone = op_iter.clone();
                let res =set_values(kvpairs,&mut op_iter);
                  if res.is_ok() {
                    let clone_peek = iter_clone.peek();
                    println!("value of key: {}",clone_peek.unwrap());
                    let len =stream.write(b"+OK\r\n");
                    println!("Sent payload of len: {}", len.unwrap());
                  }

        },
        "get" => {let key = op_iter.next().unwrap().to_owned(); 
            let res = get_values(key, kvpairs);
            if res.is_ok() {
                println!("retrieved val: {}", res.as_ref().unwrap());
                let val_stream = res.as_ref().unwrap().as_bytes();
                let payload = [b"+",val_stream,b"\r\n"].concat();
                let len =stream.write(&payload[..]);
                println!("Sent payload of len: {}", len.unwrap());
            }
            else{
                println!("error on getting val: {}",res.unwrap_err());
            }

        },


        
        _ => { }
    }
    



    

    
    




    
    





       

       
       


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
