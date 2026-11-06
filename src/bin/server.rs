use mini_redis::executor::handle_line;
use mini_redis::storage::Storage;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    println!("Listening on 127.0.0.1:6379");

    let storage = Arc::new(Mutex::new(Storage::new()));

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let storage = Arc::clone(&storage);
        thread::spawn(move || {
            println!("connecting");
            let reader = BufReader::new(stream.try_clone().unwrap());
            for line in reader.lines() {
                let line = line.unwrap();
                println!("receive: {line}");

                let reply = handle_line(&line, &mut storage.lock().unwrap());
                writeln!(stream, "{reply}").unwrap();
            }
            println!("disconnect");
        });
    }
}
