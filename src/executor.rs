use crate::command::{Command, parse_command};
use crate::storage::Storage;
use std::time::Duration;

//the input here is ParseCommand's output
//alt storage accordingly
//output string for users
pub fn execute(cmd: Command, storage: &mut Storage) -> String {
    match cmd {
        Command::Set { key, value, ex } => {
            storage.set(key, value, ex.map(Duration::from_secs));
            "OK".to_string()
        }
        Command::Get { key } => match storage.get(&key) {
            Some(value) => value,
            None => "(nil)".to_string(),
        },
        Command::Del { key } => {
            if storage.del(&key) {
                "1".to_string()
            } else {
                "0".to_string()
            }
        }
    }
}

pub fn handle_line(input: &str, storage: &mut Storage) -> String {
    match parse_command(input.trim()) {
        Ok(cmd) => execute(cmd, storage),
        Err(e) => format!("(error){e}"),
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    // ---------- 正常流程 ----------

    #[test]
    fn overwrite_value() {
        let mut s = Storage::new();
        handle_line("SET k v1", &mut s);
        handle_line("SET k v2", &mut s);
        assert_eq!(handle_line("GET k", &mut s), "v2");
    }

    #[test]
    fn get_after_del_is_nil() {
        let mut s = Storage::new();
        handle_line("SET k v", &mut s);
        handle_line("DEL k", &mut s);
        assert_eq!(handle_line("GET k", &mut s), "(nil)");
    }

    #[test]
    fn keys_are_independent() {
        let mut s = Storage::new();
        handle_line("SET a 1", &mut s);
        handle_line("SET b 2", &mut s);
        handle_line("DEL a", &mut s);
        assert_eq!(handle_line("GET a", &mut s), "(nil)");
        assert_eq!(handle_line("GET b", &mut s), "2");
    }

    #[test]
    fn command_is_case_insensitive() {
        let mut s = Storage::new();
        assert_eq!(handle_line("set k v", &mut s), "OK");
        assert_eq!(handle_line("GeT k", &mut s), "v");
        assert_eq!(handle_line("dEl k", &mut s), "1");
    }

    #[test]
    fn key_is_case_sensitive() {
        let mut s = Storage::new();
        handle_line("SET Name v", &mut s);
        assert_eq!(handle_line("GET name", &mut s), "(nil)");
    }

    #[test]
    fn extra_whitespace_and_newline() {
        let mut s = Storage::new();
        assert_eq!(handle_line("   SET   k   v   \n", &mut s), "OK");
        assert_eq!(handle_line("GET k\n", &mut s), "v");
    }

    // ---------- 过期 ----------

    #[test]
    fn set_with_ex_returns_ok_and_readable() {
        let mut s = Storage::new();
        assert_eq!(handle_line("SET k v EX 10", &mut s), "OK");
        assert_eq!(handle_line("GET k", &mut s), "v");
    }

    #[test]
    fn key_expires() {
        let mut s = Storage::new();
        handle_line("SET k v EX 1", &mut s);
        thread::sleep(Duration::from_millis(1100));
        assert_eq!(handle_line("GET k", &mut s), "(nil)");
    }

    #[test]
    fn del_expired_key_returns_0() {
        let mut s = Storage::new();
        handle_line("SET k v EX 1", &mut s);
        thread::sleep(Duration::from_millis(1100));
        assert_eq!(handle_line("DEL k", &mut s), "0");
    }

    #[test]
    fn overwrite_without_ex_clears_ttl() {
        let mut s = Storage::new();
        handle_line("SET k v1 EX 1", &mut s);
        handle_line("SET k v2", &mut s);
        thread::sleep(Duration::from_millis(1100));
        assert_eq!(handle_line("GET k", &mut s), "v2");
    }

    // ---------- 错误处理 ----------

    #[test]
    fn unknown_command() {
        let mut s = Storage::new();
        let reply = handle_line("PING", &mut s);
        assert!(reply.starts_with("(error)"));
        assert!(reply.contains("unknown command"));
    }

    #[test]
    fn wrong_number_of_args() {
        let mut s = Storage::new();
        for input in ["SET", "SET k", "GET", "GET a b", "DEL", "DEL a b"] {
            let reply = handle_line(input, &mut s);
            assert!(
                reply.contains("wrong number of arguments"),
                "input: {input}, reply: {reply}"
            );
        }
    }

    #[test]
    fn invalid_ex_values() {
        let mut s = Storage::new();
        for input in ["SET k v EX 0", "SET k v EX -5", "SET k v EX abc"] {
            let reply = handle_line(input, &mut s);
            assert!(
                reply.contains("invalid expire time"),
                "input: {input}, reply: {reply}"
            );
        }
    }

    #[test]
    fn syntax_error_on_bad_option() {
        let mut s = Storage::new();
        let reply = handle_line("SET k v PX 10", &mut s);
        assert!(reply.contains("syntax error"));
    }

    #[test]
    fn error_does_not_change_storage() {
        let mut s = Storage::new();
        handle_line("SET k v", &mut s);
        handle_line("SET k v2 EX 0", &mut s); // 无效，不应覆盖
        assert_eq!(handle_line("GET k", &mut s), "v");
    }
}
*/
