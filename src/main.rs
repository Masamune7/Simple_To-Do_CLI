/*
** Simple CLI To-Do List App **
Features:
- Add todo
- mark todo as finished
- delete todo

Future Plans: 
- app loop, 
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
    
    print!("Header: ");
    read_line(&f)?;
    
    // User input
    let input = user_input();

    f.seek(SeekFrom::End(0))?;

    // Write user input
    write_to_file(&f, &input)?;

    f.seek(SeekFrom::Start(0))?;

    print!("--- Updated File Content ---\n");
    read_all_lines(&f)?;

    Ok(())
}

fn read_line(f: &File) -> io::Result<()> {
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    if reader.read_line(&mut buffer)? > 0 {
        print!("{buffer}");
    } else {
        println!("(empty file)");
    }

    Ok(())
}

fn read_all_lines(f: &File) -> io::Result<()> {
    let reader = BufReader::new(f);
    let mut line_number = 0;

    for line in reader.lines() {
        let content = line?;
        if line_number == 0{
            println!("Head: {}", content);
        } else{
            println!("{}: {}", line_number, content);
        }
        line_number += 1;
    }

    if line_number == 0 {
        println!("(empty file)");
    }
    Ok(())
}

fn write_to_file(f: &File, input: &str) -> io::Result<()> {
    let mut writer = BufWriter::new(f);

    // Write the input followed by a newline character
    writer.write_all(input.as_bytes())?;
    writer.write_all(b"\n")?;

    writer.flush()?;
    Ok(())
}

fn user_input() -> String{
    print!("\nNew To-Do: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    input.trim().to_string()
}

// #[test]
// fn test_main(){
//     assert!("",&buffer[..n]);
// }