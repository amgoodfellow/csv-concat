extern crate glob;
use glob::glob;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Result, Write};

// Fairly self-explanatory struct to represent possible configuration options 
// pattern -> a 'glob' used to grab matching files
// dest_file_name -> By default 'all.csv'
// create_dest_file -> Right now an unused option
// append -> Should we append to <dest_file_name> or write over existing?
// from_file_column -> Should a column be added specifying which file the row came from
struct CustomOptions {
    pattern: String,
    dest_file_name: String,
    create_dest_file: bool,
    append: bool,
    from_file_column: bool,
}

// A function intended to handle arguments passed to the binary, and return a set of options
// # Arguments
// - args -- A vector of arguments passed to the binary
fn handle_args(args: &Vec<String>) -> Result<CustomOptions> {
    let help_keyword = String::from("help");
    let interactive_keyword = String::from("interactive");

    if args.contains(&help_keyword) {
        println!("csv-concat is a simple command line utility to concatenate several similar .csv files together");
        println!("USAGE:\n\tcsv-concat\n\tcsv-concat <pattern> <destination-file-name> (not-implemented)\n\tcsv-concat interactive");
        std::process::exit(0);
    } else if args.contains(&interactive_keyword) {
        start_interactive()
    } else {
        Ok(CustomOptions {
            pattern: String::from("./*.csv"),
            dest_file_name: String::from("all.csv"),
            create_dest_file: true,
            append: true,
            from_file_column: true,
        })
    }
}

// Starts an interactive REPL type thing for setting custom options with a bit more explanation
fn start_interactive() -> Result<(CustomOptions)> {
    let mut pattern = String::new();
    println!("Welcome to the interactive csv-concat tool :)\n");
    println!("Please specify a pattern you'd like to match csv files to:\nDefault: *.csv");
    print!("pattern> ");
    input_to_string(&mut pattern);
    let mut dest_file_name = String::new();
    println!("Please specify a destination file name:\nDefault: all.csv");
    print!("file-name> ");
    input_to_string(&mut dest_file_name);
    let mut choice_number = String::new();
    let mut create_dest_file = true;
    let mut append = true;
    let mut from_file_column = true;
    loop {
        println!("If you'd like to change the default values, enter the corresponding number, else type 'done'");
        println!("\t1. Create destination file ({})", create_dest_file);
        println!(
            "\t2. If destination file exists, append to the end ({})",
            append
        );
        println!("\t3. Create from-file column ({})", from_file_column);
        print!("csv-concat> ");
        input_to_string(&mut choice_number);

        match choice_number.as_ref() {
            "1" => create_dest_file = !create_dest_file,
            "2" => append = !append,
            "3" => from_file_column = !from_file_column,
            "done" => break,
            _ => println!("Invalid input. Try again"),
        }

        choice_number = String::new();
    }

    Ok(CustomOptions {
        pattern,
        dest_file_name,
        create_dest_file,
        append,
        from_file_column,
    })
}

fn input_to_string(mut user_string: &mut String) {
    let stdin = std::io::stdin();
    std::io::stdout().flush().expect("Error flushing stdout");
    stdin
        .lock()
        .read_line(&mut user_string)
        .expect("Error reading from stdin");
    trim_newline(&mut user_string);
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

fn concat_csvs(options: CustomOptions) -> Result<()> {
    let dest_file = OpenOptions::new()
        .create(true)
        .append(options.append)
        .open(&options.dest_file_name)?;

    let mut header_inserted = false;
    for entry in glob(&options.pattern).expect("Invalid pattern") {
        match entry {
            Ok(path) => {
                let file_name = path.clone();
                let file = File::open(path)?;
                let mut line_number = 0;
                for line in BufReader::new(file).lines() {
                    if header_inserted == false && line_number == 0 {
                        if options.from_file_column {
                            writeln!(&dest_file, "File,{}", line?)?;
                        } else {
                            writeln!(&dest_file, "{}", line?)?;
                        }
                        header_inserted = true;
                    } else {
                        if options.from_file_column {
                            writeln!(&dest_file, "{},{}", file_name.display(), line?)?;
                        } else {
                            writeln!(&dest_file, "{}", line?)?;
                        }
                    }
                    line_number += 1;
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let generated_options = handle_args(&args)?;

    concat_csvs(generated_options)
}
