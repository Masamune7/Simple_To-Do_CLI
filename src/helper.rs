use std::io::{self, BufRead, BufReader, BufWriter, Seek, SeekFrom, Write};
use std::fs::{File};

pub fn read_line(f: &File) -> io::Result<()> {
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    if reader.read_line(&mut buffer)? > 0 {
        print!("{buffer}");
    } else {
        println!("(empty file)");
    }

    Ok(())
}

pub fn read_all_lines(f: &mut File) -> io::Result<()> {
    f.seek(SeekFrom::Start(0))?;

    let reader = BufReader::new(f);
    let mut line_number = 0;

    println!("\n--- Current To-Do List ---");

    for line in reader.lines() {
        let content = line?;
        if line_number == 0{
            println!("\nHead: {}", content);
        } else{
            println!("{}: {}", line_number, content);
        }
        line_number += 1;
    }

    if line_number == 0 {
        println!("(empty file)");
    }

    println!("--------------------------");
    Ok(())
}

pub fn write_to_file(f: &File, input: &str) -> io::Result<()> {
    let mut writer = BufWriter::new(f);

    // Write the input followed by a newline character
    writer.write_all(input.as_bytes())?;
    writer.write_all(b"\n")?;

    writer.flush()?;
    Ok(())
}

pub fn user_input() -> String{
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    input.trim().to_string()
}