
#[allow(dead_code)]
#[derive(Debug)]
pub enum Token {
    Command(CMD),
    Keyword(String),
    Argument(ARG),
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum CMD {
    Help,
    Version,
    ListKeys,
    UnknownCMD(String),
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

#[allow(dead_code)]
#[derive(Debug)]
pub enum ARG{
    OnlyKEY,
    OnlyCMD,
    OnlyINFO,
} impl ARG {
    pub fn parse(str: &String) -> Option<ARG> {
        match str.as_str() {
            "--key" => Some(ARG::OnlyKEY),
            _ => None,
        }
    }
}
