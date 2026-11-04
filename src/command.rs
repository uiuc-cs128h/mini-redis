
/*
Command enums we want to implement
1. Set
2. Get
3. Del
*/

//If we want to store multiple types of value we can create a value enum;
/*
Valid command:
SET name CS199
SET name CS199 EX 60
GET name
DEL name
*/

//identify command
//needs to specify ParseError
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Set {key: String, value:String, ex: Option<u64>,},
    Get {key: String,},
    Del {key: String,},
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    EmptyCommand,
    UnknownCommand(String),
    WrongArgs(&'static str), //wrong number arguments
    InvalidEx(String), //invalid expiration time
    SyntaxError,
}

//implment display

pub fn parse_command(input: &str) -> Result<Command, ParseError> {
    let split: Vec<&str> = input.split_whitespace().collect();
    if split.is_empty() {
        return Err(ParseError::EmptyCommand);
    }
    let command = split[0].to_ascii_uppercase();
    match command.as_str() {
        "SET" => match split.as_slice() {
            [_cmd, k, v] => Ok(Command::Set{key: k.to_string(), value: v.to_string(), ex: None}),
            [_cmd,k,v,t,sec] => {
                if !t.eq_ignore_ascii_case("EX") {
                    return Err(ParseError::SyntaxError);
                }
                match sec.parse::<u64>() {
                    Ok(number) if number > 0 => Ok(Command::Set{
                        key: k.to_string(),
                        value: v.to_string(),
                        ex: Some(number),
                    }),
                    _ => Err(ParseError::InvalidEx(sec.to_string()))
                }
            }
            _=>Err (ParseError::WrongArgs("SET"))
        }
        "GET" => match split.as_slice() {
            [_cmd,k] => Ok(Command::Get{key: k.to_string()}),
            _=> Err(ParseError::WrongArgs("GET")),
        }
        "DEL" => match split.as_slice() {
            [_cmd,k] => Ok(Command::Del{key: k.to_string()}),
            _=> Err(ParseError::WrongArgs("DEL")),
        }
        _=> Err(ParseError::UnknownCommand(split[0].to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_set_basic() {
        assert_eq!(
            parse_command("SET name CS199"),
            Ok(Command::Set {key: "name".to_string(), value: "CS199".to_string(), ex: None,})
        );
        assert_eq!(
            parse_command("  set   key1   val1  \n"),
            Ok(Command::Set {key: "key1".to_string(), value: "val1".to_string(), ex: None,})
        );
    }

    #[test]
    fn test_parse_set_with_ex() {
        assert_eq!(
            parse_command("SET name CS199 EX 60"),
            Ok(Command::Set {key: "name".to_string(), value: "CS199".to_string(), ex: Some(60),})
        );
        assert_eq!(
            parse_command("SET name CS199 ex 10"),
            Ok(Command::Set {key: "name".to_string(), value: "CS199".to_string(), ex: Some(10),})
        );
    }

    #[test]
    fn test_parse_get_and_del() {
        assert_eq!(
            parse_command("GET name"),
            Ok(Command::Get {key: "name".to_string()})
        );
        assert_eq!(
            parse_command("DEL name"),
            Ok(Command::Del {key: "name".to_string()})
        );
    }

    #[test]
    fn test_parse_empty_and_unknown() {
        assert_eq!(parse_command(""), Err(ParseError::EmptyCommand));
        assert_eq!(parse_command("   "), Err(ParseError::EmptyCommand));
        assert_eq!(
            parse_command("PING"),
            Err(ParseError::UnknownCommand("PING".to_string()))
        );
    }

    #[test]
    fn test_parse_wrong_args() {
        assert_eq!(parse_command("SET"), Err(ParseError::WrongArgs("SET")));
        assert_eq!(parse_command("SET k"), Err(ParseError::WrongArgs("SET")));
        assert_eq!(parse_command("SET k v EX"), Err(ParseError::WrongArgs("SET")));
        assert_eq!(parse_command("DEL"), Err(ParseError::WrongArgs("DEL")));
        assert_eq!(parse_command("DEL k1 k2"), Err(ParseError::WrongArgs("DEL")));
        assert_eq!(parse_command("GET"), Err(ParseError::WrongArgs("GET")));
        assert_eq!(parse_command("GET k1 k2"), Err(ParseError::WrongArgs("GET")));  
    }

    #[test]
    fn test_parse_syntax_and_invalid_ex() {
        assert_eq!(
            parse_command("SET k v PX 60"),
            Err(ParseError::SyntaxError)
        );
        assert_eq!(
            parse_command("SET k v EX 0"),
            Err(ParseError::InvalidEx("0".to_string()))
        );
        assert_eq!(
            parse_command("SET k v EX -114514"),
            Err(ParseError::InvalidEx("-114514".to_string()))
        );
        assert_eq!(
            parse_command("SET k v EX nowak"),
            Err(ParseError::InvalidEx("nowak".to_string()))
        );
    }
}
