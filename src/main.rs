use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;

fn problem_notification(problem_message:String){
    loop {
        println!("very bad!!! {}",problem_message);
    }

}

fn main() {
    let listener =match TcpListener::bind("127.0.0.1:7878") {
        Ok(listerer) => listerer,
        Err(error) => 
        {
            println!("Fehler beim serverstart {}", error);
            return;
        }
    };

    loop{
        match listener.accept() {
            Ok((stream, adr)) => {
                println!("stream {}",adr);
                
            }
            Err(error) => {
                let stringed_error = format!("{}",error);
                problem_notification(stringed_error);
            }
        }
    }
}
