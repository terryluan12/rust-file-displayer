use std::io::{self, Write};

pub fn getInput(message: &str, input: &mut String) -> io::Result<()> {
    println!("{message}\n");
    print!("> ");
    io::stdout().flush()?;

    io::stdin().read_line(input)
        .expect("Error: Error reading input");
        
    Ok(())

}