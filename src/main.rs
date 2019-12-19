use colored::Colorize;
use scanln::scanln;

use std::env;

mod system;
mod deps;

/* TODO: Refactor, this will be only an entry point */

fn check_platfom() {
    // FIXME: This should try to cover all unix-like
    if cfg!(target_os = "linux") {
        
    } else if cfg!(target_os = "windows") {
        panic!(
            "{}",
            "Configuration tool is not supported on this OS!".red()
        );
    } else {
        panic!("{}", "Could not determine your current OS".red());
    }
}


fn main() {
    check_platfom();
    /* Arguments parsing section */
    let args: Vec<String> = env::args().collect();

    /* Arguments count */
    println!("{}", args.len());

    match args.len() {
        1 => {
            system::init()
        },
        2 => {
            match args[1].as_str() {
                "-h" | "--help" => {
                    println!(" Usage: rustscrapper [options] [quickruns]");
                    println!(" Options:");
                    println!(" -h, --help  \t show this help message and exit");
                    println!(" -l, --list  \t list available quickruns");
                    println!(" -q, --quick \t perform a (safe) quick cleanup");
                    println!(" -s, --system\t show basic system information");
                    return;
                },
                // TODO: Call functions here
                _ => {
                    println!("Invalid argument, run 'rustscrapper --help to see available options'");
                    std::process::exit(1);
                }
            }
        },

        _ => panic!("Invalid arguments or length")
    }
}
