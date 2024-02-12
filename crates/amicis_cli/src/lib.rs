use amicis_core::Greetable;
use args::{Command, HelloArgs};

pub mod args;

pub fn run(args: HelloArgs) {
    if let Some(Command::Greet { name }) = args.command {
        let greeting = name.greet();
        println!("{greeting}");
    }
}
