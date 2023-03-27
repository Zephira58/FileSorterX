#![allow(dead_code)]
#![allow(unused_must_use)]

use clap::Parser;
use std::time::SystemTime;
use FileSorterX::{create_files, sort_files};

/*
Made by Xanthus
Check out my other works at https://github.com/Xanthus58
Email me at 'Xanthus58@protonmail.com'
You can see more information on my website https://xanthus58.github.io/Xanthus58/
*/

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Sorts the files in the current directory
    #[arg(short, long, default_value_t = false)]
    sort: bool,

    /// Creates 10,000 test files in the current directory
    #[arg(short, long, default_value_t = false)]
    create: bool,

    /// Verbose mode
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Generates a log file
    #[arg(short, long, default_value_t = false)]
    log: bool,
}

fn main() {
    let start = SystemTime::now();
    let args = Args::parse();

    if args.sort {
        sort_files(args.verbose, args.log); // idk why but if i put error handling here it crashes the application. I need to fix it
        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        println!("Time taken: {:?}", duration);
    } else if args.create {
        create_files();
        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        println!("Time taken: {:?}", duration);
    } else if !args.sort && !args.create {
        println!("No arguments given. Use 'FileSorterX --help' for more information");
    }
}
