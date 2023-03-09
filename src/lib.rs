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
            match Token::parse(&str) {
                Ok(token) => results.push(token),
                Err(err) => return Err(err),
            }
        }
        Ok(results)
    }
}


