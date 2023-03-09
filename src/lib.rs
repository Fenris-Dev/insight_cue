pub mod tokens;

use std::{fs, path::Path};
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
        match cmd {
            CMD::Help => print_help(),
            CMD::Version => print_version(),
            CMD::ListKeys => print_list(&cfg),
        }

        return Ok(());
    }


    // else search for keywords, must have a keyword
    if let Some(keys) = cfg.get_keyword_tokens() {
        let indexs = cfg.data.search_for_keywords(&keys);
        let mut has_triggered = false;

        for i in indexs {
            if let Some(section) = cfg.data.sections.get(i) {
                print_data_section(&cfg, section);
                has_triggered = true;
            }
        }
        if has_triggered {
            return Ok(());
        }
    }

    // else show a message about needing keywords
    print_go_to_help();
    Ok(())
}
#[allow(dead_code)] const BLACK: &str = "\x1b[30m";
#[allow(dead_code)] const RED: &str = "\x1b[31m";
#[allow(dead_code)] const GREEN: &str = "\x1b[32m";
#[allow(dead_code)] const YELLOW: &str = "\x1b[33m";
#[allow(dead_code)] const BLUE: &str = "\x1b[34m";
#[allow(dead_code)] const MAGENTA: &str = "\x1b[35m";
#[allow(dead_code)] const CYAN: &str = "\x1b[36m";
#[allow(dead_code)] const WHITE: &str = "\x1b[37m";
#[allow(dead_code)] const RESET: &str = "\x1b[0m";

fn print_help(){
    let version = env!("CARGO_PKG_VERSION");

    println!("insight-cue {}", version);
    println!("Jquirky <@Jquirky13>");
    println!("");
    println!("insight-cue (iq) searchs the config file for a given keywords.");
    println!("by default, iq will show all information related to the keywords");
    println!("but by using options that can be filtered.");
    println!("");
    println!("Project home page: https://github.com/Jquirky/insight_cue");
    println!("");
    println!("USAGE:");
    println!("   iq [KEYWORD] [OPTIONS]");
    println!("   iq [COMMAND]");
    println!("");
    println!("ARGUMENTS");
    println!("   --key");
    println!("      shows only keys");
    println!("");
    println!("   --cmd");
    println!("      shows only commands");
    println!("");
    println!("COMMANDS:");
    println!("   --version");
    println!("      Shows the current version of insight-cue");
    println!("");
    println!("   --list");
    println!("      Lists all keywords that are within the config file");
    println!("");
    println!("   --help");
    println!("      shows the help page");
}

fn print_version(){
    let version = env!("CARGO_PKG_VERSION");
    println!("insight-cue {}", version)
}

fn print_list(cfg: &Config){
    let mut shown = Vec::new();
    println!("List keys within config:");

    for s in &cfg.data.sections  {
        for k in &s.keywords {
            if shown.contains(k) {
                continue;
            }
            println!(" - {}{}{}", MAGENTA,k,RESET);
            shown.push(k.clone());
        }
    }

}

fn print_go_to_help(){
    println!("{}error{}: no valid keyword or arguments were provided", RED, RESET);
    println!("For a list of keywords try --list");
    println!("For more information try --help");
}

fn print_data_section(cfg: &Config, section: &Section){

    let mut title = String::new();
    for (i, element) in section.keywords.iter().enumerate() {
        title.push_str(MAGENTA);
        title.push_str(element);
        title.push_str(RESET);
        if i != section.keywords.len() - 1 {
            title.push_str(", ")
        }
    }
    println!("{}", title);

    if let Some(keys) = &section.keys{
        println!(" {}Keys{}:", RED, RESET);
        for (k, i) in keys {
            let key_format = format!("{}[{}]{}:", GREEN, k, RESET);
            println!("  {:<30}\"{}\"", key_format, i)
        }
    }
    if let Some(commands) = &section.commands{
        println!(" {}Commands{}:", RED, RESET);
        for (c, i) in commands {

            let cmd_format = format!("{}[{}]{}:", GREEN, c, RESET);
            println!("  {:<30}\"{}\"", cmd_format, i)
        }
    }
    println!("");
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

    let data = generate_default_data();
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
} impl Data {
    pub fn search_for_keywords(&self, keywords: &Vec<&String>) -> Vec<usize>{
        let mut results = Vec::new();
        for (i, sect) in self.sections.iter().enumerate() {
            for word in keywords {
                if sect.keywords.contains(word) {
                    results.push(i);
                }
            }
        }
        return results;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Section {
    keywords: Vec<String>,
    keys: Option<Vec<(String, String)>>,
    commands: Option<Vec<(String, String)>>,
}

pub fn generate_default_data() -> Data {
    let iq = Section {
        keywords : vec!["iq".to_string()],
        keys: None,
        commands: Some(vec![
                       ("--help".to_string(), "shows help menu".to_string())
        ])
    };

    Data {
        sections: vec![iq]
    }
}
