use amicis_core::Greetable;
use args::{Command, HelloArgs};

pub mod args;

pub fn run(args: HelloArgs) {
    match args.command {
        Some(Command::Greet { name }) => {
            let greeting = name.greet();
            println!("{greeting}");
        }
        _ => {}
    }
}
