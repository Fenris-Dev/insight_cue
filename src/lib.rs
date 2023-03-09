pub mod tokens;

use std::{fs::{File, self}, env, path::Path, io::{BufWriter, Write}, fmt::format};
use directories::ProjectDirs;
use crate::tokens::*;
use serde::{Deserialize, Serialize};

// There are commands, such as --help, --version
// There are options, such as --key, --command
// There is the keyword input
//
// - a command will override anything else

pub fn run(cfg: Config) -> Result<(), Box<Error>> {

    // check for command first
    if let Some(cmd) = cfg.get_first_command_token() {
        println!("CMD: {:?}", cmd);

        return Ok(());
    }


    // else search for keywords, must have a keyword
    if let Some(keys) = cfg.get_keyword_tokens() {
        println!("keys: {:?}", keys);

    }


    // else show a message about needing keywords


    if false {
        for i in &cfg.tokens {
            println!("Got: {:?}", i);
        }
    }

    Ok(())
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

fn check_or_create_file() -> Result<(), Error>{
    let Some(proj_dirs) = ProjectDirs::from("dev", "Jquirky", "insight-cue") else { 
        return Err(Error::new(String::from("Cant find config directory")));
    };
    let dir:&Path = proj_dirs.config_dir();
    let file = dir.join("config.json");
    if dir.exists() && file.exists() {
        return Ok(());
    }

    if let Err(err) = fs::create_dir_all(dir) {
        return Err(Error::new(format!("Failed to create config dir: {}", err)));
    }

    let data = Data{sections:vec![]};
    let data:Result<String, serde_json::Error> = serde_json::to_string_pretty(&data);
    if let Err(err) = data {
        return Err(Error::new(format!("Failed to json: {}", err)));
    }
    let data = data.unwrap();

    let result = fs::write(file, data);
    if let Err(err) = result {
        return Err(Error::new(format!("Failed to create config: {}", err)));
    }

    return Ok(());
}

fn load_file() -> Option<String> {
    let Some(proj_dirs) = ProjectDirs::from("dev", "Jquirky", "insight-cue") else { 
        return None;
        //return Err(Error::new(String::from("Failed to locate config directory")))
    };
    let file = proj_dirs.config_dir();
    let file = file.join("config.json");

    let result = fs::read_to_string(file);
    if result.is_err(){ 
        return None;
    }

    return Some(result.unwrap());
}

fn seralize_conifg() -> Result<Data, Box<dyn std::error::Error>>{
    let result = load_file().unwrap_or("".to_string());
    let data: Data = serde_json::from_str(&result)?;
    Ok(data)
}
    

#[derive(Debug)]
pub struct Config {
    tokens: Vec<Token>,
    data: Data,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, Error> { 
        args.next(); // This next() call, passes over the program path
        let tokens = parse_tokens(args)?;
       
        //CREATE FIRST IF NONE!
        match check_or_create_file() {
            Err(err) => return Err(Error::new(format!("Serialize Err: {}", err))),
            _ => (),
        }

        //THEN LOAD DATA
        let data = match seralize_conifg() {
            Ok(d) => d,
            Err(err) => return Err(Error::new(format!("Deserialize Err: {}",err.to_string()))),
        };

        Ok(Self {
            tokens,
            data
        })
    }

    fn get_first_command_token(&self) -> Option<&CMD> {
        for i in &self.tokens {
            match i {
                Token::Command(t) => return Some(t),
                _ => continue,
            }
        }
        None
    }

    fn get_keyword_tokens(&self) -> Option<Vec<&String>> {
        let mut keys = Vec::new();

        for i in &self.tokens {
            match i {
                Token::Keyword(w) => keys.push(w),
                _ => continue,
            }
        }
        if keys.is_empty() {
            return None;
        }
        Some(keys)
    }

}
#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    sections: Vec<Section>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Section {
    keywords: Option<Vec<(String, String)>>
}
