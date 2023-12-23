use std::fs;
use dialoguer::Input;
use clap::{App, Arg};
use rand::{Rng, thread_rng};



fn main() {
    let matches = App::new("File Renamer")
        .version("1.0")
        .author("Your Name. <your_email@example.com>")
        .about("Renames files in a directory")
        .arg(Arg::with_name("path")
             .short('p')
             .long("path")
             .value_name("DIRECTORY_PATH")
             .help("Sets the directory path")
             .takes_value(true))
        .get_matches();

    let path = matches.value_of("path").unwrap_or(".");

    // Request the file path from the user
    let path = Input::<String>::new()
        .with_prompt("Please enter directory path")
        .default(".".to_string())
        .interact()
        .unwrap();

    println!("path {path}");

    // Count the number of items in the directory
    let path_clone = path.clone();
    let item_count = fs::read_dir(path_clone)
        .unwrap()
        .count();

    println!("item_count: {item_count}");

    // Initialize a counter to track the number of files renamed
    let mut counter = 0;

    // Iterate over the items in the directory
    for (index, entry) in fs::read_dir(path).unwrap().enumerate() {
        println!("--");
        let entry = entry.unwrap(); 

        println!("index {index} __ counter {counter} ");

        let path = entry.path();

        // Check if the item is a file
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            

            // Define the valid file extensions
            let file_exts = [
                ".tif", 
                ".exr", 
                ".jpg", 
                ".png", 
                ".jpeg", 
                ".gif", 
                ".webp"
            ];

            // Check if the file has a valid extension
            if file_exts.iter().any(|ext| file_name.ends_with(ext)) {
                
                // Generate a random number
                let random_number = generate_random_number().to_string();

                // Get the file extension
                let file_ext = file_name.split(".").last().unwrap();

                // Construct the new file name
                let new_file_name = random_number + "." + file_ext;

                println!("oldie --> new: {file_name} -- > {new_file_name} " );
                if file_name.eq(&new_file_name){ 
                    println!("....EQUAL...");
                    
                } else {
                    // Construct the new file path
                    let new_path = path.with_file_name(new_file_name);

                    // Rename the file
                    fs::rename(path, new_path).unwrap();
                    counter  = counter +1;
                }
            }
        }
    }
}

// Function to generate a random number
fn generate_random_number() -> u64 {
    let mut rng = thread_rng();
    rng.gen::<u64>()
}

