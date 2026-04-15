mod command_handlers;

use std::env::args;

use command_handlers::{status_handler, help_handler};

fn main() {

    let args: Vec<String> = args().collect();
    
    if args.len() < 2 {
        help_handler::handle_help_command();
        return;
    }
    
    let command = &args[1];

    match command.as_str() {
        "help" => {
            help_handler::handle_help_command();
        },
        "status" => {
            status_handler::handle_status_command();
        },
        _ => {
            println!("Unknown command: {}", command);
            help_handler::handle_help_command();
        }
    }
}
