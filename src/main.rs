//! pylinefix Fix string wrapping in Python code
//!
//! Someone had to write a process that could safely wrap python lines in a way
//! consistent with PEP-8 and for some reason black and other formatters can't.
//! So this is a CLI that can do it and I can wrap that in a macro in vim and
//! VSCode.
//!
//! The process takes a single line as input via stdin and wraps the python
//! string at the 80th column by default, but can be overridden with the '-l'
//! argument. Word boundaries are respected when wrapping the string and indents
//! are assumed to be spaces. I won't support tabs for religious reasons.
//!
//! # Examples
//!
//! ```
//! echo '    error = "this is a ridiculously long explanation that needs to be wrapped appropriately in order for it to be PEP-8",' | pylinefix
//! ```
//! Produces
//!
//! ```
//!    error = (
//!        "this is a ridiculously long explanation that needs to be wrapped "
//!        "appropriately in order for it to be PEP-8"
//!    ),
//! ```


use clap::Parser;
use std::io::Read;



/// Program Arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 80)]
    linewrap: u8,

    #[arg(short, long, default_value_t = '"')]
    str_char: char
}


/// Main entry point
fn main() -> Result<(), std::io::Error> {
    // Parse arguments
    let args = Args::parse();
    let mut stdin = std::io::stdin();
    let line: String = {
        // Copy stdin into a UTF-8 String
        let mut buf: Vec<u8> = Vec::new();
        stdin.read_to_end(&mut buf)?;
        String::from_utf8(buf).expect("Failed to read string as valid UTF-8").trim_end().into()
    };
    // Doesn't need to be wrapped, just print it as-is and exit.
    if line.len() < args.linewrap.into() {
        print!("{}", line);
        return Ok(())
    }
    // Determine the current indentation level
    let indent_level = count_leading_spaces(&line);

    // Strings are UTF-8 character sequences and must treat each character as an
    // element for display in lieu of bytes.
    let line_as_chars: Vec<char> = line.chars().collect();
    // A buffer to hold the current ascii-character sequence as a word.
    // Non-ascii characters are considered word boundaries which are safe to
    // wrap at.
    let mut word = String::new();
    // Keep track of how many characters on the current line have been printed.
    let mut current_col: usize = 0;
    // Indentation string for each line we wrap
    let prefix = (0..(indent_level+4)).map(|_| ' ').collect::<String>();
    // Keep track of when we encounter the first string delimiter, defaults to `"`
    let mut first_quote: bool = false;

    // Iterate over each character appropriately
    for c in line_as_chars {
        current_col += 1;
        // We need to wrap, print the quote, newline, indent, and new quote
        if current_col >= (args.linewrap as usize) {
            print!("{}\n{}{}", args.str_char, prefix, args.str_char);
            current_col = prefix.len() + 1;
        }
        if c == args.str_char {
            if !first_quote {
                // First quote encountered, add a '(' then wrap
                print!("(\n{}{}", prefix, c);
                first_quote = true;
                current_col = prefix.len();
            } else {
                // Flush our current word buffer
                print!("{}", word);
                word.clear();
                // Close off the parenthesis
                // print <quote><newline><indent>)
                print!("{}\n{})", c, (0..indent_level).map(|_| ' ').collect::<String>());
            }
        } else {
            word.push(c);
        }
        // Word boundary is safe to print on current line
        if !c.is_ascii_alphanumeric() {
            // flush
            print!("{}", word);
            word.clear();
        }
    }
    // Terminate the whole output with a newline
    println!("");
    Ok(())
}


fn count_leading_spaces(s: &str) -> usize {
    s.chars().take_while(|&c| c == ' ').count()
}
