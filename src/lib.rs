use std::collections::HashMap;
use std::time::Instant;

lazy_static::lazy_static! {
    pub static ref VERSION: &'static str = {
        let mut version = env!("CARGO_PKG_VERSION").to_string();
        if let Some(v) = option_env!("GIT_HASH") {
            version.push_str("-");
            version.push_str(v);
        }
        if cfg!(debug_assertions) {
            version.push_str("-debug");
        }
        Box::leak(Box::new(version))
    };
}

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod commands;
use commands::Command;
mod executer;
mod parser;

pub fn execute(input: &[String]) -> Result<(), String> {
    let mut app = clap::App::new("ArmaLint").version(*crate::VERSION).arg(
        clap::Arg::with_name("debug")
            .global(true)
            .help("Turn debugging information on")
            .long("debug"),
    );

    let mut commands: Vec<Box<dyn Command>> = Vec::new();
    let mut hash_commands: HashMap<String, &Box<dyn Command>> = HashMap::new();

    // Add commands here
    commands.push(Box::new(crate::commands::Execute {}));

    for command in commands.iter() {
        let sub = command.register();
        hash_commands.insert(sub.get_name().to_owned(), command);
        app = app.subcommand(sub);
    }

    let matches = app.get_matches_from(input);

    let start = if matches.is_present("time") {
        Some(Instant::now())
    } else {
        None
    };

    match matches.subcommand_name() {
        Some(v) => match hash_commands.get(v) {
            Some(c) => {
                let sub_matches = matches.subcommand_matches(v).unwrap();
                c.run(sub_matches)?;
            }
            None => println!("No Command"),
        },
        None => println!("No Command"),
    }

    if matches.is_present("time") {
        let elapsed = start.unwrap().elapsed();
        println!("Execution took {} ms", elapsed.as_millis());
    }

    Ok(())
}
