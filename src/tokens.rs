use std::fmt::Display;


#[derive(Debug, PartialEq)]
pub enum Token {
    Command(CMD),
    Keyword(String),
    Argument(ARG),
} impl Token {
    pub fn parse(str: &String) -> Result<Token, Error> {

        if let Some(keyword) = Self::parse_keyword(&str) {
            return Ok(keyword);
        }

        if let Some(cmd) = CMD::parse(&str) {
            return Ok(Token::Command(cmd));
        }

        if let Some(arg) = ARG::parse(&str) {
            return Ok(Token::Argument(arg));
        }

        return Err(Error::new(format!("\u{001b}[31mUnknown Argument: {}\u{001b}[0m", str)));
    }

    fn parse_keyword(str: &String) -> Option<Token>{
        match Self::is_keyword(&str) {
            true => Some(Token::Keyword(str.to_owned())),
            false => None,
        }
    }

    fn is_keyword(arg: &String) -> bool {
        !arg.starts_with("--")
    }
}

#[derive(Debug, PartialEq)]
pub enum CMD {
    Help,
    Version,
    ListKeys,
} impl CMD {
    pub fn parse(str: &String) -> Option<CMD> {
        match str.as_str() {
            "--help" => Some(CMD::Help),
            "--version" => Some(CMD::Version),
            "--list" => Some(CMD::ListKeys),
            _ => None
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ARG{
    OnlyKEY,
    OnlyCMD,
    OnlyINFO,
} impl ARG {
    pub fn parse(str: &String) -> Option<ARG> {
        match str.as_str() {
            "--key" => Some(ARG::OnlyKEY),
            "--cmd" => Some(ARG::OnlyCMD),
            "--info" => Some(ARG::OnlyINFO),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Error {
    message: String,
} impl Error {
    pub fn new(message: String) -> Self{
        Self {
            message
        }
    }
    pub fn description(&self) -> &str {
        &self.message
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error { }



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn get_token_cmd() {
        let result = Token::parse(&String::from("--help"));
        if result.is_err() {
            assert!(false, "Failed to parse String into Token CMD");
        }
        let result = result.unwrap();

        let expected = Token::Command(CMD::Help);
        assert_eq!(expected, result);
    }

    #[test]
    fn get_token_arg() {
        let result = Token::parse(&String::from("--key"));
        if result.is_err() {
            assert!(false, "Failed to parse String into Token ARG");
        }
        let result = result.unwrap();

        let expected = Token::Argument(ARG::OnlyKEY);
        assert_eq!(expected, result);
    }

    #[test]
    fn get_token_key() {
        let result = Token::parse(&String::from("nvim"));
        if result.is_err() {
            assert!(false, "Failed to parse String into Token key");
        }
        let result = result.unwrap();

        let expected = Token::Keyword(String::from("nvim"));
        assert_eq!(expected, result);
    }

    #[test]
    fn dont_get_token_key() {
        let result = Token::parse(&String::from("--nvim"));
        if result.is_ok() {
            assert!(false, "Got Token key, when shouldnt have");
        }
    }

    #[test]
    fn should_get_cmd() {
        let result = CMD::parse(&String::from("--help"));
        if result.is_none() {
            assert!(false, "Failed to parse String into CMD Token");
        }
        let result = result.unwrap();
        assert_eq!(CMD::Help, result);
    }

    #[test]
    fn shouldnt_get_cmd() {
        let result = CMD::parse(&String::from("help"));
        if result.is_some() {
            assert!(false, "Got CMD Token, when shouldnt have");
        }
    }
    
    #[test]
    fn should_get_arg() {
        let result = ARG::parse(&String::from("--key"));
        if result.is_none() {
            assert!(false, "Failed to parse String into ARG Token");
        }
        let result = result.unwrap();
        assert_eq!(ARG::OnlyKEY, result);
    }

    #[test]
    fn shouldnt_get_arg() {
        let result = ARG::parse(&String::from("key"));
        if result.is_some() {
            assert!(false, "Got ARG Token, when shouldnt have");
        }
    }    

    #[test]
    fn should_get_token_keyword_1() {
        let result = Token::parse(&String::from("hello-world"));
        if result.is_err() {
            assert!(false, "Failed to parse String into Token key");
        }
        let result = result.unwrap();

        let expected = Token::Keyword(String::from("hello-world"));
        assert_eq!(expected, result);
    }
    
    #[test]
    fn should_get_token_keyword_2() {
        let result = Token::parse(&String::from("hello--world"));
        if result.is_err() {
            assert!(false, "Failed to parse String into Token key");
        }
        let result = result.unwrap();

        let expected = Token::Keyword(String::from("hello--world"));
        assert_eq!(expected, result);
    }
}
