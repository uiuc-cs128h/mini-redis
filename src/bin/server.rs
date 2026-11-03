use std::io;
use std::time::Duration;
use mini_redis::command::{parse_command, Command, ParseError};
use mini_redis::storage::Storage;

fn main() 
{
    let mut line = String::new();
    let mut storage = Storage::new();
    loop 
    {
        let cur_read = match io::stdin().read_line(&mut line) 
        {
            Ok(n) => n,
            Err(e) => 
            {
                eprintln!("Can't Read: {e}");
                break;
            }
        };
        if cur_read == 0 // Ctrl+D
        {
            break; 
        }
        let input = line.trim();
        match parse_command(input) 
        {
            Ok(Command::Set { key, value, ex }) => 
            {
                storage.set(key, value, ex.map(Duration::from_secs));
                println!("OK");
            }
            Ok(Command::Get { key }) => match storage.get(&key) 
            {
                Some(value) => println!("{value}"),
                None => println!("(nil)"),
            },
            Ok(Command::Del { key }) => 
            {
                if storage.del(&key)
                {
                    println!("1");
                }
                else
                {
                    println!("0");
                }
            }
            Err(ParseError::EmptyCommand) => continue, 
            Err(e) => println!("(error) {e}"),
        }
        line.clear();
    }
}
