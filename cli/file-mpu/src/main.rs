use std::*;
use std::fs::*;
use std::io::{Write, Read};

fn main() {

    let args: Vec<String> = env::args().collect();

    let command = args[1].clone();

    match command.as_str() {
        "create" => {
            let file = args[2].clone();
            let cont = args[3].clone();

            create_file(&file, &cont)
                .expect("Error");
        } 
        _ => {
            eprintln!("Unknown command: {}", command);
            std::process::exit(1);
    }

    // create_file("poem.txt", "Abhinav Agarawalla")
    //     .expect("Error calling the function"); 

    // read_file("poem.txt") 
    //     .expect("Error calling the functon"); 
    
}

fn create_file(filename: &str, content: &str) -> Result<(), std::io::Error> {
    match File::create(filename) {
        Ok(mut file) => {
            println!("File created");
            if let Err(err)= file.write_all(content.as_bytes()) {
                eprintln!("Error writing data : {}",err);
                return Err(err);
            }
            println!("Data added");
            Ok(())
        }
        Err(err) => {
            eprintln!("Error creating the file: {}",err);
            Err(err)
        }
    }
}

fn read_file(filename: &str) -> Result<(), std::io::Error> {
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            if let Err(err)= file.read_to_string(&mut content){
                eprintln!("Error reading the file: {}",err);
            }
            println!("{:#?}",&content);
            Ok(())
        }
        Err(err) => {
            eprintln!("Error opening the file: {}",err);
            Err(err)
        }
    }
}

fn rename_file(old_name: &str, new_name: &str) -> Result<(), std::io::Error> {
    match fs::rename(old_name, new_name) {
        Ok(()) => {
            println!("{} renamed to {}", old_name, new_name);
            Ok(())
        }
        Err(err) => {
            eprintln!("Error renaming the file: {}",err);
            Err(err)
        }
    }
}

fn copy_file(source: &str, destination: &str) -> Result<(), std::io::Error> {
    match fs::copy(source, destination) {
        Ok(_) => {
            println!("{} copied in {}", source, destination);
            Ok(())
        }
        Err(err) => {
            eprintln!("Error copying the file: {}",err);
            Err(err)
        }
    }
}

fn delete_file(filename: &str) -> Result<(), std::io::Error> {
    match fs::remove_file(filename) {
        Ok(()) => {
            println!("File deleted");
            Ok(())
        }
        Err(err) => {
            eprintln!("Error deletiing the file: {}",err);
            Err(err)
        }
    }
}

}