use std::env;
use colored::Colorize;

mod parser;
mod info;

fn get_tag(args: &Vec<String>, tag: &str) -> usize {
    let position = args.iter().position(|x| x == tag);
    let tag_index = match position {
        Some(index) => index,
        None => {
            eprintln!("{}: Incorrect usage.", "error".red());
            eprintln!("Correct usage: {} --file file.las --info info", "laswear".cyan());
            panic!("Incorrect usage.");
        }
    };
    if tag_index == args.len() - 1 {
        eprintln!("{}: Incorrect usage.", "error".red());
        eprintln!("Correct usage: {} --file file.las --info info", "laswear".cyan());
        panic!("Incorrect usage.");
    }
    tag_index + 1
}

fn asking_for_help(args: &Vec<String>) -> bool {
    let help_exists = args.iter().any(|x| x == "--help");
    help_exists
}

fn print_help() {
    println!("Expected usage:\n");
    println!("   {} --file <file> --info <info>\n", "laswear".cyan());
    println!("\t {} - The file to be processed. Only .las files are currently supported.", "--file".magenta());
    println!("\t {} - The information that is parsed from the file. [{}]", "--info".magenta(), "Optional".bold());
    println!("\t\t - {} : Stastics describing the distribution of the classified points.", "classification".bold());
    println!("\t\t - {} : Statistics describing the distribution of the ground point.", "ground".bold());
    println!("");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if asking_for_help(&args) {
        print_help();
        return;
    }

    let file_name_index = get_tag(&args, "--file");
    let file_name = &args[file_name_index];
    let las_file: parser::LasFile = match parser::read_file(&file_name) {
        Ok(parsed_file) => parsed_file,
        Err(e) => panic!("Encountered error while parsing file: {}", e),
    };

    let info_index = get_tag(&args, "--info");
    let info = &args[info_index];

    info::extract_file_info(&las_file, info);
    
    println!("Successfully parsed {} with {} points", file_name, las_file.header.number_point_records);
}
