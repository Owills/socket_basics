extern crate tokio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::Value;
use serde::{Deserialize, Serialize};
use serde_json::{Result};
use std::env;
use std::fs;
use std::sync::Arc;
use tokio::net::TcpStream;
use futures::executor::block_on;
use tokio_native_tls::{ TlsConnector };



#[allow(non_snake_case)]
#[tokio::main]
async fn main() -> Result<()> {

    //comand z
    let mut tcp = true;
    let mut port = "27993";
    let mut ip = "proj1.3700.network";
    let mut nu = "o.maker";
    //comand line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.iter().any(|i| i=="-p")  {
        let index = args.iter().position(|r| r == "-p").unwrap();
        port = &args[index +1];
    } else if args.iter().any(|i| i=="-s") {
        tcp = false;
        port = "27994";
    }
    if args.len() >= 2 {
        ip = &args[args.len()-2];
        nu = &args[args.len()-1];
    }
    
    let mut counter = 0;

    // read word_list
    let word_list = fs::read_to_string("wordlist.txt")
        .expect("LogRocket: Should have been able to read the file");
    
    if tcp {
        // tcp connection
        let socket = TcpStream::connect(ip.to_owned() + ":" + port).await;
        
        let mut socket = match socket {
            Ok(v) => {
                //println!("[+] Successfully connected");
                v
            }
            Err(_) => {
                println!("ERROR could not connect to the server");
                std::process::exit(-1);
            }
        };
        
        // send hello message
        let hello = "{\"type\": \"hello\",\"northeastern_username\": \"".to_owned() + &nu +"\"}\n";
        socket.write(hello.as_bytes()).await.unwrap();

        let mut buf = vec![0;1000000];
        let mut t = RType::Start;
        loop {
            let res;
            
            // recieve messages from servers
            match socket.read(&mut buf).await {
                Ok(0) => {
                    println!("port: {}", port);
                    std::process::exit(1);
                }
                Ok(_n) => {
                    let bc = buf.clone();
                    res = String::from_utf8(bc).unwrap();
                    //println!("[+] Server responded with {}",get_only_data(&res));
                }
                Err(_) => {
                    panic!("[-] Some fatal error occured");
                }
            }


            let response: Bye = serde_json::from_str(get_only_data(&res))?;
            if response.flag != None {
                println!("{}",response.flag.unwrap());
                std::process::exit(1);
            }
            
            

            match t {
                // read start message and make first guess
                RType::Start => {
                    let response: Start = serde_json::from_str(get_only_data(&res))?;
                    let guess = "{\"type\": \"guess\",\"id\": \"".to_owned() + &response.id +"\", \"word\": \"aahed\"}\n";
                    socket.write(guess.as_bytes()).await.unwrap();
                    t = RType::Retry;
                }
                RType::Retry => {
                    
                    let response: Response = serde_json::from_str(get_only_data(&res))?;
                    let guess = "{\"type\": \"guess\",\"id\": \"".to_owned() + &response.id +"\", \"word\": \"" + &nextguess(&word_list, &response) + "\"}\n";
                    socket.write(guess.as_bytes()).await.unwrap();
                    counter = counter + 1;
                    
                }    
            }
        }


    } else {
        // tls connection
        use native_tls::TlsConnector;
        use std::io::{Read, Write};
        use std::net::TcpStream;

        let connector = TlsConnector::new().unwrap();

        let socket = TcpStream::connect(ip.to_owned() + ":" + port).unwrap();
        let mut socket = connector.connect(ip, socket).unwrap();

        let hello = "{\"type\": \"hello\",\"northeastern_username\": \"".to_owned() + &nu +"\"}\n";
        socket.write(hello.as_bytes()).unwrap();

        let mut buf = vec![0;1000000];
        let mut t = RType::Start;
        loop {
            let res;
            // recieve messages from servers
            match socket.read(&mut buf) {
                Ok(0) => {
                    println!("port: {}", port);
                    std::process::exit(1);
                }
                Ok(_n) => {
                    let bc = buf.clone();
                    res = String::from_utf8(bc).unwrap();
                    //println!("[+] Server responded with {}",get_only_data(&res));
                }
                Err(_) => {
                    panic!("[-] Some fatal error occured");
                }
            }

            let response: Bye = serde_json::from_str(get_only_data(&res))?;
            if response.flag != None {
                println!("{}",response.flag.unwrap());
                std::process::exit(1);
            }
            
            match t {
                // read start message and make first guess
                RType::Start => {
                    let response: Start = serde_json::from_str(get_only_data(&res))?;
                    let guess = "{\"type\": \"guess\",\"id\": \"".to_owned() + &response.id +"\", \"word\": \"aahed\"}\n";
                    socket.write(guess.as_bytes()).unwrap();
                    t = RType::Retry;
                }
                RType::Retry => {
                    let response: Response = serde_json::from_str(get_only_data(&res))?;
                    let guess = "{\"type\": \"guess\",\"id\": \"".to_owned() + &response.id +"\", \"word\": \"" + &nextguess(&word_list, &response) + "\"}\n";
                    socket.write(guess.as_bytes()).unwrap();
                }    
            }
        }
    }
}


fn nextguess<'a>(wordlist : &String, response: &'a Response) -> String {
    let mut word = "zzzzz";

    let mut char_vec: Vec<char> = word.chars().collect();
    let alphabet = vec!["a", "b", "c", "d", "e", "f", "g", "h" , "i", "j", "k", "l" , "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"  ];
    

    let guesses = &response.guesses;
    let marks = &guesses[guesses.len()-1].marks;
    

    let word = &guesses[guesses.len()-1].word;

    let guess: Vec<char> = guesses[guesses.len()-1].word.chars().collect();
    let mut counter = 0;
    for j in 0..marks.len() {
        if marks[j] == 0 || marks[j] == 1 {
            let index = alphabet.iter().position(|&r| r == guess[j].to_string()).unwrap();
            let mut i = 1;
            let mut x = None;
            while x == None {
                x =  wordlist.find(&("\n".to_owned() + &word[..counter] + alphabet[index+i]));
                i = i + 1;
            }
            return wordlist[x.unwrap()+1..x.unwrap()+6].to_string();
        } else {
            counter = counter + 1;
        }
    }

    let word: String = char_vec.into_iter().collect();
    return word;
}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// used to determine what type of response to send back to the server
#[derive(PartialEq)]
enum RType {
    Start,
    Retry,
}

//stores data from the 'start' type response
#[derive(Serialize, Deserialize)]
struct Start {
    id : String,
    r#type: String,
}

//stores data from the 'bye' type response
#[derive(Serialize, Deserialize)]
struct Bye {
    id : String,
    r#type: String,
    flag: Option<String>,
}

// stores data from the 'retry' type response
#[derive(Serialize, Deserialize)]
struct Response {
    id : String,
    r#type: String,
    guesses: Vec<Info>,
}

#[derive(Serialize, Deserialize)]
struct Info {
    word : String,
    marks : Vec<i8>,
}

// remove trailing date, including newline and (random bytes I think added from buffer)
// allows serde to parse json data into 'Start' and 'Response' structs
fn get_only_data(s: &String) -> &str {
    if s.contains("\"flag\"") {
        let pos = s.find('}');
        return &s[0..pos.unwrap()+1];
    }
    let pos = s.rfind('}');
    return &s[0..pos.unwrap()+1];
}


