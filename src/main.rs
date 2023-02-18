use std::iter::Peekable;
// Uncomment this block to pass the first stage
 use std::net::{TcpListener,TcpStream};
 use std::slice::Iter;
use std::sync::{Mutex, Arc};
use std::{str, u8, i32};
 use std::io::{BufReader,Read,Write, BufRead};
 use std::thread;
 use std::collections::HashMap;

 
 fn encode() {

 }

fn decode() {

}



fn set_values( kvmap: Arc<Mutex<HashMap<String, String>>>, kv :&mut Peekable<Iter<String>>) -> Result<Option<String>, &'static str>{
    
    let values = kv.clone();
    if values.len() < 2 {
        return Err("no valid key");
    }
    let  key = kv.next().unwrap().to_owned();
    let val = kv.next().unwrap().to_string().to_owned();
    
    println!("set_values method: key: {}, value: {}", key.clone(),val.clone());
    if let Ok(mut kvp1) = kvmap.lock(){
        kvp1.insert( key.to_owned(), val.to_owned());
        let  map_value  = kvp1.get(&key);
        let x =map_value.as_deref();
        

        return Ok(Some(x.unwrap().to_owned()));
    }
    else{
        Err("Could not lock mutex")
    }
 


      
}

fn get_values(key: String, kvmap: Arc<Mutex<HashMap<String, String>>>) -> Result<String, &'static str> {

    if let Ok( kvp1) = kvmap.lock(){
        let value = kvp1.get(&key);
        
    if value.is_none(){
        return Err("value is not in map");
    }

    println!("retrieved value is: {}",value.unwrap());
    return Ok(value.unwrap().to_string());
    }
    else{
        Err("error in locking mutex")
    }

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

    
    
    fn conn_handler( stream: &mut TcpStream,kvpairs: Arc<Mutex<HashMap<String,String>>>) {
        
        
        let mut buf = [0;512]; 
        let mut reader = BufReader::new(stream.try_clone().unwrap());

        //loop  {
        let res = reader.read(&mut buf).unwrap();     

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
    
    
    println!("operation: {}", operation);
    match operation {
        "ping"  => {let len =stream.write(b"+PONG\r\n");
        println!("Sent payload of len: {}", len.unwrap());
    },

        "echo" => {
                   let packet =[b"+", op_vec[1].as_bytes(), b"\r\n"].concat();        
                   stream.write(&packet[..]);
                  },

        "set" => { 
                
                let mut iter_clone = op_iter.clone();
                let res =set_values(kvpairs,&mut op_iter);
                
                  if res.is_ok() {
                    iter_clone.next();
                    let clone_peek = iter_clone.peek();
                    println!("value of key: {}, value in map: {}",clone_peek.unwrap(), res.unwrap().unwrap());
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
    


      //}; 
    }


     let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
     let  kvpairs: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
     for stream in listener.incoming() {
        match stream {
            
             Ok( mut succ_stream) => {
                 println!("accepted new connection");
                 let arc_kvpairs_clone = Arc::clone(&kvpairs);
                 thread::spawn(move ||  {
                 conn_handler( &mut succ_stream,  arc_kvpairs_clone);
                 });
                
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}
