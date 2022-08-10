use std::env;
use colored::Colorize;

mod parser;

fn get_file_index(args: &Vec<String>) -> usize {
    let position = args.iter().position(|x| x == "--file");
    let file_index = match position {
        Some(index) => index,
        None => {
            eprintln!("{}: Incorrect usage.", "error".red());
            eprintln!("Correct usage: {} --file file.las", "laswear".cyan());
            panic!("No --file argument found.");
        }
    };
    if file_index == args.len() - 1 {
        panic!("No file specified.");
    }
    file_index + 1
}

fn asking_for_help(args: &Vec<String>) -> bool {
    let help_exists = args.iter().any(|x| x == "--help");
    help_exists
}

fn print_help() {
    println!("Expected usage:\n");
    println!("   {} --file <file> --info <info\n", "laswear".cyan());
    println!("\t {} - The file to be processed. Only .las files are currently supported.", "--file".magenta());
    println!("\t {} - {}, The information that is parsed from the file. Options are:", "--info".magenta(), "Optional".bold());
    println!("\t\t - {} : Stastics describing the distribution of the classified points.", "classification".bold());
    println!("\t\t - {} : Statistics describing the distribution of the ground point.", "ground".bold());
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if asking_for_help(&args) {
        print_help();
        return;
    }

    let file_index = get_file_index(&args);
    let file_name = &args[file_index];
    let las_file: parser::LasFile = match parser::read_file(file_name) {
        Ok(parsed_file) => parsed_file,
        Err(e) => panic!("Encountered error {}", e),
    };
    println!("Successfully parsed {} with {} points", file_name, las_file.header.number_point_records);
}
