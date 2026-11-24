use mini_redis::executor::handle_line;
use mini_redis::storage::Storage;
use std::io;

fn main() {
    let mut line = String::new();
    let mut storage = Storage::new();
    loop {
        line.clear();
        let cur_read = match io::stdin().read_line(&mut line) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Can't Read: {e}");
                break;
            }
        };
        if cur_read == 0
        // Ctrl+D
        {
            break;
        }
        /*
        if line.trim().is_empty() {
            continue;
        }
         */
        println!("{}", handle_line(&line, &mut storage));
    }
}
