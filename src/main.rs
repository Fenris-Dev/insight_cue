use std::{env, process};

fn main() {

    //first arg is the target, e.g the application
    let cfg = insight_cue::Config::build(env::args()).unwrap_or_else(|err| {
        eprint!("Err: {}", err.description());
        process::exit(1);
    });
    

    if let Err(err) = insight_cue::run(cfg) {
        eprintln!("Err: {}", err.description());
        process::exit(1);
    }
}
