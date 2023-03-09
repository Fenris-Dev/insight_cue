pub mod tokens;

use crate::tokens::*;


// There are commands, such as --help, --version
// There are options, such as --key, --command
// There is the keyword input
//
// - a command will override anything else

pub fn run(cfg: Config) -> Result<(), Box<Error>> {
    dbg!(cfg);

    Ok(())
}

#[derive(Debug)]
pub struct Config {
    tokens: Vec<Token>,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, Error> {
    
        args.next(); // This next() call, passes over the program path
        let tokens = Self::parse_tokens(args)?;

        Ok(Self {
            tokens
        })
    }
    
    fn parse_tokens(args: impl Iterator<Item = String>) -> Result<Vec<Token>, Error> {
        let mut results = Vec::new();

        for str in args {

            // TODO impl this into Token as a method ?
            if let Some(keyword) = Self::parse_keyword(&str) {
                results.push(keyword);
                continue;
            }

            if let Some(cmd) = CMD::parse(&str) {
                results.push(Token::Command(cmd));
                continue;
            }

            if let Some(arg) = ARG::parse(&str) {
                results.push(Token::Argument(arg));
                continue;
            }

            return Err(Error::new(format!("\u{001b}[31mUnknown Argument: {}\u{001b}[0m", str)));
        }

        Ok(results)
    }

    fn parse_keyword(str: &String) -> Option<Token>{
        match Self::is_keyword(&str) {
            true => Some(Token::Keyword(str.to_owned())),
            false => None,
        }
    }

    fn is_keyword(arg: &String) -> bool {
        !arg.contains("--")
    }
}

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
