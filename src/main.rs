extern crate glob;
use glob::glob;
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Result, Write};

fn main() -> Result<()> {
    let dest_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("all.csv")?;

    let mut header_inserted = false;
    for entry in glob("./*.csv").expect("Invalid pattern") {
        match entry {
            Ok(path) => {
                if path.file_stem() != Some(OsStr::new("all")) {
                    let file_name = path.clone();
                    let file = File::open(path)?;
                    let mut line_number = 0;
                    for line in BufReader::new(file).lines() {
                        if !header_inserted && line_number == 0 {
                            let mut csv_line = line?.clone();
                            if csv_line.starts_with('\u{FEFF}') {
                                csv_line.remove(0);
                            }
                            writeln!(&dest_file, "File,{}", csv_line)?;
                            header_inserted = true;
                        } else if !header_inserted || line_number != 0 {
                            writeln!(&dest_file, "{},{}", file_name.display(), line?)?;
                        }
                        line_number += 1;
                    }
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(())
}
