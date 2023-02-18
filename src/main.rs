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



//fn set_values(  kvmap: &mut Arc<Mutex<HashMap<String, String>>>, kv :&mut Peekable<Iter<String>>) -> Result<Option<String>, &'static str>{
    fn set_values( mut kvmap: &mut HashMap<String, String>, kv :&mut Peekable<Iter<String>>) -> Result<Option<String>, &'static str>{  
    let values = kv.clone();
    if values.len() < 2 {
        return Err("no valid key");
    }

    
    
    let mut iter = kv.clone();
    let  key = iter.next().unwrap();
    println!("value of op_iter at key index: {}",iter.peek().unwrap());
    let val = iter.next().unwrap().to_string().to_owned();
    println!("value of op_iter at val ind: {}",iter.peek().unwrap());

    kvmap.insert( key.to_owned(), val.to_owned());
    let def = "cannot set value".to_owned();
    let  map_value  = kvmap.get(key).unwrap_or(&def);
    return Ok(Some(map_value.clone()));
    
    
    
    // if let Ok(mut kvp1) = kvmap.lock(){
    //     let  key = iter.next().unwrap();

    //     let val = iter.next().unwrap().to_string().to_owned();

    //     kvp1.insert( key.to_owned(), val.to_owned());
        
    //     let def = "cannot set value".to_owned();
    //     let  map_value  = kvp1.get(key).unwrap_or(&def);
    //     return Ok(Some(map_value.clone()));
    // }
    // else{
    //     Err("Could not lock mutex")
    // }


    

      
}

// fn get_values(key: String, kvmap: Arc<Mutex<HashMap<String, String>>>) -> Result<String, &'static str> {
    fn get_values(key: String, kvmap: &mut HashMap<String, String>) -> Result<String, &'static str> {
        let err_msg = "invalid key".to_owned();
        let value = kvmap.get(&key).unwrap_or(&err_msg); 

        return Ok(value.to_string());      
    
    
    // if let Ok( kvp1) = kvmap.lock(){
    //     let err_msg = "invalid key".to_owned();
    //     let value = kvp1.get(&key).unwrap_or(&err_msg);

    // println!("retrieved value is: {}",value);
    // return Ok(value.to_string());
    // }
    // else{
    //     Err("error in locking mutex")
    // }

}
 
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
        let mut newkvpair = kvpairs.clone();
        let mut test_map = std::collections::HashMap::<String, String>::new();
        loop  {
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
                
                //let mut iter_clone = op_iter.clone();
                
                let res =set_values(&mut test_map,&mut op_iter);
                
                  if res.is_ok() {
                    //iter_clone.next();
                   // let clone_peek = iter_clone.peek().unwrap().clone();
                    
                    stream.write(b"+OK\r\n");
                    for key in test_map.keys(){
                        println!("keys set method: {key}");
                    }
                    
                    
                    
                  }

        },
        "get" => {let mut keyval = op_iter.next().unwrap().clone();
            keyval = op_iter.next().unwrap().clone();
            println!("map size in get: {}", test_map.len());
            for key in test_map.keys(){
                println!("keys set method: {key}");
            }

            let res = get_values(keyval,&mut test_map);
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



fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
     let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
     let mut kvpairs: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
     for stream in listener.incoming() {
        match stream {
            
             Ok( mut succ_stream) => {
                 println!("accepted new connection");
                 let arc_kvpairs_clone = Arc::clone(&kvpairs);
                 thread::spawn(move ||  conn_handler( &mut succ_stream,  arc_kvpairs_clone) );
                
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}
