
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
use std::fmt;
impl fmt::Display for ParseError 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        match self 
        {
            ParseError::EmptyCommand => write!(f,"empty command"),
            ParseError::UnknownCommand(cmd) => write!(f,"unknown command '{cmd}'"),
            ParseError::WrongArgs(cmd) => write!(f,"wrong number of arguments for '{cmd}'"),
            ParseError::InvalidEx(t) => write!(f,"invalid expire time '{t}'"),
            ParseError::SyntaxError => write!(f,"syntax error"),
        }
    }
}

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


    /*
    match split.as_slice() {
        [command, k] => {
            match command.to_ascii_uppercase().as_str() {
                "GET" => Ok(Command::Get{key: k.to_string()}),
                "DEL" => Ok(Command::Del{key: k.to_string()}),
                _ => Err(ParseError::InvalidArguments),
            }
        }
        [command, k, v] => {
            match command.to_ascii_uppercase().as_str() {
                "SET" => Ok(Command::Set{key: k.to_string(), value: v.to_string(), ex: None}),
                _ => Err(ParseError::InvalidArguments),
            }
        }
        [command, k, v, command_ex, sec] => {
            match command.to_ascii_uppercase().as_str() {
                "SET"=> {
                    if command_ex != 
                }
            }
        }
        _ => Err(ParseError::InvalidArguments),
    }
    */
    /*
    let command: String = split[0].to_uppercase();
    match command.as_str() {
        "SET" => {
            match split.len() {
                3 => Ok(Command::Set{key: split[1].to_string(), value: split[2].to_string(), ex: None}),
                5=>{
                    if !split[3].eq_ignore_ascii_case("EX") {
                        return Err(ParseError::InvalidArguments);
                    }
                    match split[4].parse::<u64>() {
// Should 0 be allowed???
                        Ok(number) if number > 0 => Ok(Command::Set{
                            key: split[1].to_string(),
                            value: split[2].to_string(),
                            ex: Some(number),
                        }),
                        _ => Err(ParseError::InvalidArguments)
                    }
                }
                _ => Err(ParseError::InvalidArguments),
            }
        }
        "GET" => {
            if split.len() != 2 {
                return Err(ParseError::InvalidArguments);
            }
            Ok(Command::Get{key: split[1].to_string()})
        }
        "DEL" => {
            if split.len() != 2 {
                return Err(ParseError::InvalidArguments);
            }
            Ok(Command::Del{key: split[1].to_string()})
        }
        _ => Err(ParseError::UnknownCommand),
    }
}
    */
