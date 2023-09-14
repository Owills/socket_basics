extern crate tokio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::io::{AsyncRead, AsyncWrite};
use serde::{Deserialize, Serialize};
use serde_json::{Result};
use std::env;
use std::sync::Arc;
use tokio::net::TcpStream;
////use tokio_rustls::rustls::{ClientConfig, OwnedTrustAnchor, RootCertStore, ServerName};
//use tokio_rustls::TlsConnector;
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
    dbg!(&args);
    
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
    
    println!("port: {}", port);
    println!("ip: {}", ip);
    println!("nuid: {}", nu);

    
    
    if tcp {
        // tcp connection
        let socket = TcpStream::connect(ip.to_owned() + ":" + port).await;
        
        let mut socket = match socket {
            Ok(v) => {
                println!("[+] Successfully connected");
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

        let mut buf = vec![0;1024];
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
                    println!("[+] Server responded with {}",get_only_data(&res));
                }
                Err(_) => {
                    panic!("[-] Some fatal error occured");
                }
            }
            match t {
                // read start message and make first guess
                RType::Start => {
                    let response: Start = serde_json::from_str(get_only_data(&res))?;
                    let guess = "{\"type\": \"guess\",\"id\": \"".to_owned() + &response.id +"\", \"word\": \"wived\"}\n";
                    socket.write(guess.as_bytes()).await.unwrap();
                    t = RType::Guess1;
                }
                // make 7 maunally guesses (start, guess1... guess6) with all 26 letters of the alphabet
                RType::Guess1 => {
                    let response: Response = serde_json::from_str(get_only_data(&res))?;
                    let guess = "{\"type\": \"guess\",\"id\": \"".to_owned() + &response.id +"\", \"word\": \"homes\"}\n";
                    socket.write(guess.as_bytes()).await.unwrap();
                    t = RType::Guess2;
                }
                RType::Guess2 => {
                    let response: Response = serde_json::from_str(get_only_data(&res))?;
                    let guess = "{\"type\": \"guess\",\"id\": \"".to_owned() + &response.id +"\", \"word\": \"quiet\"}\n";
                    socket.write(guess.as_bytes()).await.unwrap();
                    t = RType::Guess3;
                }
                RType::Guess3 => {
                    let response: Response = serde_json::from_str(get_only_data(&res))?;
                    let guess = "{\"type\": \"guess\",\"id\": \"".to_owned() + &response.id +"\", \"word\": \"zebra\"}\n";
                    socket.write(guess.as_bytes()).await.unwrap();
                    t = RType::Guess4;
                }
                RType::Guess4 => {
                    let response: Response = serde_json::from_str(get_only_data(&res))?;
                    let guess = "{\"type\": \"guess\",\"id\": \"".to_owned() + &response.id +"\", \"word\": \"flick\"}\n";
                    socket.write(guess.as_bytes()).await.unwrap();
                    t = RType::Guess5;
                }
                RType::Guess5 => {
                    let response: Response = serde_json::from_str(get_only_data(&res))?;
                    let guess = "{\"type\": \"guess\",\"id\": \"".to_owned() + &response.id +"\", \"word\": \"proxy\"}\n";
                    socket.write(guess.as_bytes()).await.unwrap();
                    t = RType::Guess6;
                }
                RType::Guess6 => {
                    let response: Response = serde_json::from_str(get_only_data(&res))?;
                    let guess = "{\"type\": \"guess\",\"id\": \"".to_owned() + &response.id +"\", \"word\": \"jingo\"}\n";
                    socket.write(guess.as_bytes()).await.unwrap();
                    t = RType::Retry;
                }
                RType::Retry => {

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

        let mut buf = vec![0;1024];
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
                    println!("[+] Server responded with {}",get_only_data(&res));
                }
                Err(_) => {
                    panic!("[-] Some fatal error occured");
                }
            }
            match t {
                // read start message and make first guess
                RType::Start => {
                    let response: Start = serde_json::from_str(get_only_data(&res))?;
                    let guess = "{\"type\": \"guess\",\"id\": \"".to_owned() + &response.id +"\", \"word\": \"wived\"}\n";
                    socket.write(guess.as_bytes()).unwrap();
                    t = RType::Guess1;
                }
                // make 7 maunally guesses (start, guess1... guess6) with all 26 letters of the alphabet
                RType::Guess1 => {
                    let response: Response = serde_json::from_str(get_only_data(&res))?;
                    let guess = "{\"type\": \"guess\",\"id\": \"".to_owned() + &response.id +"\", \"word\": \"homes\"}\n";
                    socket.write(guess.as_bytes()).unwrap();
                    t = RType::Guess2;
                }
                RType::Guess2 => {
                    let response: Response = serde_json::from_str(get_only_data(&res))?;
                    let guess = "{\"type\": \"guess\",\"id\": \"".to_owned() + &response.id +"\", \"word\": \"quiet\"}\n";
                    socket.write(guess.as_bytes()).unwrap();
                    t = RType::Guess3;
                }
                RType::Guess3 => {
                    let response: Response = serde_json::from_str(get_only_data(&res))?;
                    let guess = "{\"type\": \"guess\",\"id\": \"".to_owned() + &response.id +"\", \"word\": \"zebra\"}\n";
                    socket.write(guess.as_bytes()).unwrap();
                    t = RType::Guess4;
                }
                RType::Guess4 => {
                    let response: Response = serde_json::from_str(get_only_data(&res))?;
                    let guess = "{\"type\": \"guess\",\"id\": \"".to_owned() + &response.id +"\", \"word\": \"flick\"}\n";
                    socket.write(guess.as_bytes()).unwrap();
                    t = RType::Guess5;
                }
                RType::Guess5 => {
                    let response: Response = serde_json::from_str(get_only_data(&res))?;
                    let guess = "{\"type\": \"guess\",\"id\": \"".to_owned() + &response.id +"\", \"word\": \"proxy\"}\n";
                    socket.write(guess.as_bytes()).unwrap();
                    t = RType::Guess6;
                }
                RType::Guess6 => {
                    let response: Response = serde_json::from_str(get_only_data(&res))?;
                    let guess = "{\"type\": \"guess\",\"id\": \"".to_owned() + &response.id +"\", \"word\": \"jingo\"}\n";
                    socket.write(guess.as_bytes()).unwrap();
                    t = RType::Retry;
                }
                RType::Retry => {
                    let response: Response = serde_json::from_str(get_only_data(&res))?;

                }    
            }
        }
    }
}
//test

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// used to determine what type of response to send back to the server
#[derive(PartialEq)]
enum RType {
    Start,
    Guess1,
    Guess2,
    Guess3,
    Guess4,
    Guess5,
    Guess6,
    Retry,
}

//stores data from the 'start' type response
#[derive(Serialize, Deserialize)]
struct Start {
    id : String,
    r#type: String,
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
    let pos = s.rfind('}');
    return &s[0..pos.unwrap()+1];
}


