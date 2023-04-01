#![allow(dead_code)]
#![allow(unused_must_use)]

use clap::{Parser, Subcommand};
use dotenv::dotenv;
use std::{env, path::PathBuf, process::Command, time::SystemTime};
use uuid::Uuid;
use FileSorterX::*;

/*
Made by Xanthus
Check out my other works at https://github.com/Xanthus58
Email me at 'business@xanthus.uk'
You can see more information on my website https://xanthus.uk
*/

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sorts the files in the current directory
    #[command(subcommand)]
    command: Option<Commands>,

    /// Disables telemetry
    #[arg(short, long, default_value_t = false)]
    disable_telemetry: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Sorts files based on file extension matching our database
    Sort {
        /// The input directory
        #[arg(short, long)]
        inputdir: String,

        /// The output directory
        #[arg(short, long)]
        outputdir: String,

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
    /// Creates a specified amount of files
    Create {
        /// The amount of files to create
        #[arg(short, long)]
        amount: u32,
    },
    /// Sorts files based on custom file extensions
    Customsort {
        /// The input directory
        #[arg(short, long)]
        inputdir: String,

        /// The output directory
        #[arg(short, long)]
        outputdir: String,

        /// The file extension to sort
        #[arg(short, long)]
        extension: String,

        /// Verbose mode
        #[arg(short, long, default_value_t = false)]
        verbose: bool,

        /// Generates a log file
        #[arg(short, long, default_value_t = false)]
        log: bool,
    },
    /// Updates FileSorterX to the latest version based on the github repo
    Update {},
    /// Note: Only run in a new empty directory. Runs a benchmark test
    Benchmark {},
}

fn main() {
    let cli = Cli::parse();
    if !cli.disable_telemetry {
        dotenv().ok();
        collect_telemetry();
    }

    let start = SystemTime::now();
    match &cli.command {
        Some(Commands::Sort {
            inputdir,
            outputdir,
            nesting_level,
            use_alt,
            verbose,
            log,
        }) => {
            let in_dir = PathBuf::from(inputdir);
            let out_dir = PathBuf::from(outputdir);

            if !in_dir.is_dir() {
                panic!("Provided path is not a directory: '{:?}'", in_dir)
            }

            sort_files(in_dir, out_dir, *nesting_level, *use_alt, *verbose, *log);
            let end = SystemTime::now();
            let duration = end.duration_since(start).unwrap();
            println!("Time taken: {:?}", duration);
        }
        Some(Commands::Customsort {
            inputdir,
            outputdir,
            extension,
            verbose,
            log,
        }) => {
            custom_sort(inputdir, outputdir, extension, *verbose, *log);
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
        Some(Commands::Benchmark { .. }) => {
            println!("Time Taken: {:?}", benchmark());
        }
        None => println!("No command provided. Use 'filesorterx --help' for more information."),
    }
}

fn collect_telemetry() {
    let os = env::consts::OS;
    let token = std::env::var("TELEMETRY_TOKEN").expect("TELEMETRY_TOKEN not set");
    let mut command = String::new();

    if os == "windows" {
        command = "curl -UserAgent".to_string();
    } else if os == "linux" {
        command = "curl -A".to_string();
    }

    command.push_str(&token);
    println!("{}", command);
    Command::new("curl")
        .arg(token)
        .output()
        .expect("Failed to execute command");
}
