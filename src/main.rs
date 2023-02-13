use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
};

fn parse_md_file(_file: &str) {
    println!("[ INFO ] Trying to parse {}...", _file);

    let input_filename = Path::new(_file);

    // Try to open the file
    let file = File::open(&input_filename).expect("[ ERROR ] Failed to open file!");

    let mut ptag: bool = false;
    let mut htag: bool = false;
    let mut etag: bool = false;

    let mut tokens: Vec<String> = Vec::new();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_contents = line.unwrap();

        let mut first_char: Vec<char> = line_contents.chars().take(1).collect();

        // Now check the first character to for headings
        let mut s = String::new();
        let slice = &line_contents.to_string();
        match first_char.pop() {
            Some('#') => {
                if ptag {
                    ptag = false;
                    s.push_str("</p>\n"); // adding \n for instructional clarity
                }
                if htag {
                    htag = false;
                    s.push_str("</h1>\n"); // close it if we're already open
                }

                htag = true;
                s.push_str("<h1>");
                s.push_str(&slice[2..]); // Get all but the first two characters
            }

            _ => {
                if htag {
                    htag = false;
                    s.push_str("</h1>\n");
                }

                if !ptag {
                    ptag = true;
                    s.push_str("<p>");
                }

                s.push_str(&slice);
            }
        }

        // At the very end, check if any of the tag bools are still open. If so,
        // close them.
        if htag {
            htag = false;
            s.push_str("</h1>\n");
        }

        if ptag {
            ptag = false;
            s.push_str("</p>\n");
        }

        // Don't push blank lines
        if s != "<p></p>\n" {
            tokens.push(s);
        }
    }

    // Create an output file based on the input file, minus ".md"
    let _output_filename = &_file[.._file.len() - 3];
    let mut output_filename = String::from(_output_filename);
    output_filename.push_str(".html");

    let mut outfile =
        File::create(output_filename.to_string()).expect("[ ERROR ] Could not create output file!");

    for line in &tokens {
        outfile
            .write_all(line.as_bytes())
            .expect("[ ERROR ] Could not write to output file!");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => println!("No arguments"),
        2 => parse_md_file(&args[1]),
        _ => println!("Too many arguments"),
    }
}
