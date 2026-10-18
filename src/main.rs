/*
** Simple CLI To-Do List App **
Features:
- View Todo
- Add Todo

Future Plans: 
- implement Update,
- implement delete 
*/

mod helper;
use std::io::{self, BufRead, BufReader, Write, Seek, SeekFrom};
use std::path::Path;
use std::fs::{self, OpenOptions, File};
use helper::{read_line, read_all_lines, write_to_file, user_input};

fn main() -> io::Result<()> {
    let mut status = true;
    let filename = "todo.txt";

    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename)?;

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
            "1" => {
                let mut f = OpenOptions::new().read(true).open(filename)?;
                read_all_lines(&mut f)?
            }
            "2" => {
                let mut f = OpenOptions::new().read(true).write(true).open(filename)?;
                create(&mut f)?;
                read_all_lines(&mut f)?;
            }
            "3" => {
                print!("Old Task: ");
                let context = user_input();
                update(filename, &context)?;

                let mut f = OpenOptions::new().read(true).open("todo.txt")?;
                read_all_lines(&mut f)?;
            },
            "4" => {
                print!("Task: ");
                let context = user_input();
                delete(filename, &context)?;

                let mut f = OpenOptions::new().read(true).open("todo.txt")?;
                read_all_lines(&mut f)?;
            }
            _ => {
                println!("\n[X] Invalid option! Please enter a number between 0 and 4")
            }
        }
    }    
      
    Ok(())
}

fn create(f: &mut File) -> io::Result<()> {
    println!("\n--- Create New Task ---");
    f.seek(SeekFrom::Start(0))?;
    print!("Current Header: ");
    read_line(&*f)?;
    
    // User input
    print!("New To-Do item: ");
    let input = user_input();
    
    f.seek(SeekFrom::End(0))?;
    write_to_file(f, &input)?;

    print!("\n[✓] Task successfully added!");

    Ok(())
}

fn update(path: &str, context: &str) -> io::Result<()> {
    let path_real = Path::new(path);
    let path_tmp = path_real.with_extension("tmp");

    let f_src = File::open(path_real)?;
    let reader = BufReader::new(f_src);

    let target_context = context.to_lowercase().trim().to_string();
    let mut found_line = false;
    let mut empty = true;

    {

        let mut f_tmp = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path_tmp)?; 

        for line in reader.lines() {
            empty = false;
            let content = line?;
            let clean_content = content.to_lowercase();

            if !found_line && clean_content.trim().contains(&target_context) {
                found_line = true;
                print!("Update task: ");
                let new_line = user_input();
                writeln!(f_tmp, "{}", new_line)?;
                continue;
            }

            writeln!(f_tmp, "{}", content)?;
        }
        
        f_tmp.flush()?;

    }

    if empty {
        println!("(empty file)");
    }

    fs::rename(path_tmp, path_real)?;

    if found_line {
        println!("\n[✓] Task successfully updated!");
    } else {
        println!("\n[X] Task not found!");
    }

    println!("--------------------------");
    Ok(())
}

fn delete(path: &str, context: &str) -> io::Result<()>{
    let path_real = Path::new(path);
    let path_tmp = path_real.with_extension("tmp");

    let f_src = File::open(path_real)?;
    let reader = BufReader::new(f_src);

    let target_context = context.to_lowercase().trim().to_string();
    let mut found_line = false;
    let mut empty = true;

    {
        let mut f_tmp = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path_tmp)?; 

        for line in reader.lines() {
            empty = false;
            let content = line?;
            let clean_content = content.to_lowercase();

            if !found_line && clean_content.trim().contains(&target_context) {
                found_line = true;
                continue; 
            }

            writeln!(f_tmp, "{}", content)?;
        }
        f_tmp.flush()?;
    }

    if empty {
        println!("(empty file)");
    }


    fs::rename(path_tmp, path_real)?;

    if found_line {
        println!("\n[✓] Task successfully deleted!");
    } else {
        println!("\n[X] Task not found!");
    }

    println!("--------------------------");
    Ok(())
}

// #[test]
// fn test_main(){
//     assert!("",&buffer[..n]);
// }