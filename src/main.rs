use std::iter::Peekable;
// Uncomment this block to pass the first stage

use tokio;

 use std::net::{TcpListener,TcpStream};
 use std::slice::Iter;
use std::sync::{Mutex, Arc};
use std::time::{Duration, Instant};
use std::{str, u8};
 use std::io::{BufReader,Read,Write, BufRead};
 use std::thread;
 use std::collections::HashMap;

enum RespLiterals{
    NULLBULKSTRING,
    NULLARRAY,
    OK,
    EMPTYSTRING,
    EMPTYARRAY,

}

struct Times{
     start: std::time::Instant,
    end: Option<std::time::Duration>,
}
 
 fn encode() {

 }

fn decode() {

}


struct RedisVal {
    value: String,
    timer: Option<std::time::Instant>,
    endTime : Option<std::time::Duration>
}


fn timer_flag_match(flag: Option<&String>, dur: Option<&String>) -> Option<Times> {

    let flag_unwr = flag?.to_string();

    match flag_unwr.as_str() {
        "EX" => {
            let str_dur = dur?.to_string().parse::<u64>().unwrap();
            let parsed_dur = Duration::from_secs(str_dur);
            Some(Times{start: Instant::now() , end: Some(parsed_dur) })
        },

        "PX" =>{
            let str_dur = dur?.to_string().parse::<u64>().unwrap();
            let parsed_dur = Duration::from_millis(str_dur);
            Some(Times{start: Instant::now() , end: Some(parsed_dur) })
        },
        _ => None
    }
    
    }
    


fn set_values(  kvmap:  Arc<Mutex<HashMap<String, RedisVal>>>, kv :&mut Peekable<Iter<String>>) -> Result<Option<String>, &'static str>{
    //fn set_values( mut kvmap: &mut HashMap<String, String>, kv :&mut Peekable<Iter<String>>) -> Result<Option<String>, &'static str>{  
    let values = kv.clone();
    if values.len() < 2 {
        return Err("no valid key");
    }


    let mut iter = kv;    
    if let Ok(mut kvp1) = kvmap.lock(){
        iter.next();
        let  key = iter.next().unwrap();

        let val = iter.next();
        let mut insertedVal: RedisVal = RedisVal { value: val.unwrap().to_owned() , timer: None, endTime: None};  
        
        let timer_flag = iter.next();
        let duration = iter.next();      
        let timer_info =timer_flag_match(timer_flag, duration);
        println!("entering");

        if timer_info.is_some() {
            let unw_tim = timer_info.unwrap();
            insertedVal.timer = Some(unw_tim.start);
            insertedVal.endTime = unw_tim.end;
            
            kvp1.insert( key.to_owned(), insertedVal);
        }
        let  map_value  = &kvp1.get(key).unwrap().value;
        //return Ok(Some(map_value.clone()));
        return Ok(Some(map_value.to_string()));
    }
    else{
        Err("Could not lock mutex")
    }     
}


 fn get_values(key: String, kvmap: Arc<Mutex<HashMap<String, RedisVal>>>) -> Result<Option<RedisVal>, &'static str> {
    //
        // let err_msg = "invalid key".to_owned();
        // let value = kvmap.get(&key).unwrap_or(&err_msg); 

        // return Ok(value.to_string());      

    if let Ok( kvp1) = kvmap.lock(){

        let value = kvp1.get(&key).unwrap();
        let ret_value = RedisVal { value: value.value.clone(), timer: value.timer.clone(), endTime: None };
    return Ok(Some(ret_value));
    }
    else{
        Err("error in locking mutex")
    }
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


fn conn_handler( stream: &mut TcpStream,kvpairs: Arc<Mutex<HashMap<String,RedisVal>>>) {
        
        
        let mut buf = [0;512]; 
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut newkvpair = kvpairs.clone();
        //let mut test_map = std::collections::HashMap::<String, String>::new();
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
                
                
                
                let res =set_values(  newkvpair.clone(),&mut op_iter);
                let elems: Vec<&String> =op_iter.clone().collect();
                println!("current elems in op_iter{:?}", &elems[0..elems.len()-1]);
                op_iter.next();
                  if res.is_ok() {
                    //iter_clone.next();
                   // let clone_peek = iter_clone.peek().unwrap().clone();
                    
                    stream.write(b"+OK\r\n");
                    
                    
                    
                  }

        },
        "get" => {
            op_iter.next();
            let keyval2 = op_iter.next().unwrap().clone();
            
            

            let res = get_values(keyval2,newkvpair.clone());
                if res.is_ok() {
                    let redis_opt = res.unwrap();
                // check if there is a timer
                    let redis_val = redis_opt.unwrap();
                    let curr_time = Instant::now();

                //check if 

                //send byte stream
               
            }
            else{
                println!("error on getting val: {}","vsdf");
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

     let mut kvpairs: Arc<Mutex<HashMap<String, RedisVal>>> = Arc::new(Mutex::new(HashMap::new()));

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
