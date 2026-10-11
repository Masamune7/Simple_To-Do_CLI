/*
** Simple CLI To-Do List App **

technology:
-std:io
-std:fs
-crossterm (Terminal Functionalities)
-jiff (Date Time)
-serde
-serde_json

Features:
- Add todo
- mark todo as finished
- delete todo

Future Plans: 
- app loop, 
- string matching, 
- string saving
*/

use std::io::{self, BufRead, BufReader, BufWriter, Seek, SeekFrom, Write};
use std::fs::{OpenOptions, File};
fn main() -> io::Result<()> {
    // Open file with read, write, and create capabilities
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("todo.txt")?;
    
    // Prepare reader and read first line
    print!("Existing line: ");
    read_file(&f)?;
    
    // User input
    print!("Input Text: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Write user input
    write_file(&f, input)?;

    // Rewind to the start of the file to read from the beginning
    f.seek(SeekFrom::Start(0))?;
    print!("Updated file content: ");
    read_file(&f)?;

    Ok(())
}

fn read_file(f: &File) -> io::Result<()>  {
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    if reader.read_line(&mut buffer)? > 0 {
        print!("{buffer}"); // print! avoids double newlines since buffer already has \n
    } else {
        println!("(empty file)");
    }

    Ok(())
}

fn write_file(mut f: &File, input: String) -> io::Result<()>{
    let mut writer = BufWriter::new(&mut f);
    writer.write_all(input.as_bytes())?;
    writer.flush()?;
    
    Ok(())
}

// #[test]
// fn test_main(){
//     assert!("",&buffer[..n]);
// }