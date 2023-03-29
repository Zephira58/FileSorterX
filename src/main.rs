#![allow(dead_code)]
#![allow(unused_must_use)]

use clap::{Parser, Subcommand};
use FileSorterX::{create_files, custom_sort, sort_files, update_filesorterx};
use std::{path::PathBuf, time::SystemTime};

/*
Made by Xanthus
Check out my other works at https://github.com/Xanthus58
Email me at 'Xanthus58@protonmail.com'
You can see more information on my website https://xanthus58.github.io/Xanthus58/
*/

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sorts the files in the current directory
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Sort {
        /// The input directory
        #[arg(short, long)]
        input: String,

        /// The output directory
        #[arg(short, long)]
        output: String,

        /// Number of directory levels (1-3)
        #[arg(short, long, default_value_t = 2)]
        nesting_level: u8,

        /// Use alternative sorting directory name
        #[arg(short, long, default_value_t = false)]
        use_alt: bool,

        /// Verbose mode
        #[arg(short, long, default_value_t = false)]
        verbose: bool,

        /// Generates a log file
        #[arg(short, long, default_value_t = false)]
        log: bool,
    },
    Create {
        /// The amount of files to create
        #[arg(short, long)]
        amount: u32,
    },
    Customsort {
        /// The input directory
        #[arg(short, long)]
        input: String,

        /// The output directory
        #[arg(short, long)]
        output: String,

        /// The file extension to sort
        #[arg(short, long)]
        extension: String,
    },
    Update {},
}

fn main() {
    let start = SystemTime::now();
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Sort {
            input,
            output,
            nesting_level,
            use_alt,
            verbose,
            log,
        }) => {
            let in_dir = PathBuf::from(input);
            let out_dir = PathBuf::from(output);

            if !in_dir.is_dir() {
                panic!("Provided path is not a directory: '{:?}'", in_dir)
            }

            sort_files(in_dir, out_dir, *nesting_level, *use_alt, *verbose, *log);
            let end = SystemTime::now();
            let duration = end.duration_since(start).unwrap();
            println!("Time taken: {:?}", duration);
        }
        Some(Commands::Customsort {
            input,
            output,
            extension,
        }) => {
            custom_sort(input, output, extension);
        }
        Some(Commands::Create { amount }) => {
            create_files(amount + 1);
            let end = SystemTime::now();
            let duration = end.duration_since(start).unwrap();
            println!("Time taken: {:?}", duration);
        }
        Some(Commands::Update { .. }) => {
            update_filesorterx().expect("Failed to update FileSorterX");
        }
        None => println!("No command provided. Use 'filesorterx --help' for more information."),
    }
}
