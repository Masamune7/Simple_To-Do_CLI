/*
** Simple CLI To-Do List App **
Features:
- View Todo
- Add Todo

Future Plans: 
- implement Update,
- implement delete 
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
    
    let mut status = true;

    while status {
        println!("\n=============================================================");
        println!("  OPTIONS: 1.View | 2.Create | 3.Update | 4.Delete | 0.Exit   ");
        println!("=============================================================");

        print!("Select action:");
        let input = user_input();
        
        match input.as_str() {
            "0" => {
                println!("\nExiting...");
                status = false;
            }
            "1" => read_all_lines(&mut f)?,
            "2" => create(&mut f)?,
            "3" => update(),
            "4" => delete(),
            _ => {
                println!("\n[X] Invalid option! Please enter a number between 0 and 4")
            }
        }
    }    
      
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

fn read_all_lines(f: &mut File) -> io::Result<()> {
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

fn write_to_file(f: &File, input: &str) -> io::Result<()> {
    let mut writer = BufWriter::new(f);

    // Write the input followed by a newline character
    writer.write_all(input.as_bytes())?;
    writer.write_all(b"\n")?;

    writer.flush()?;
    Ok(())
}

fn user_input() -> String{
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    input.trim().to_string()
}

fn create(f: &mut File)-> io::Result<()> {
    println!("\n--- Create New Task ---");
    f.seek(SeekFrom::Start(0))?;
    print!("Current Header: ");
    read_line(&*f)?;
    
    // User input
    print!("New To-Do item: ");
    let input = user_input();
    
    f.seek(SeekFrom::End(0))?;

    // Write user input
    write_to_file(f, &input)?;

    print!("\n[✓] Task successfully added!");
    read_all_lines(f)?;

    Ok(())
}

fn update(){
    println!("\n[!] Update not implemented yet")
}

fn delete(){
    println!("\n[!] Delete not implemented yet")
}

// #[test]
// fn test_main(){
//     assert!("",&buffer[..n]);
// }