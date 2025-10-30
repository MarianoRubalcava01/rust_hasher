
//imports the std "crate" (library) for us to use in our main()
use std::env;
use std::fs;
use sha2::{Sha256, Digest};
use hex;



//rust allows you to return a result on main function 
//the result can be one of two things, either a pass or an error
fn main() -> Result<(), Box<dyn std::error::Error>>{

    //init variable named arg 
    //arg will use the args() function to take in arguments from the CLI
    //.collect() will "collect" all items from the list
    //and put them into our collection Vec<String>
    //our args vector will have 2 strings "./rust_hasher" our program name 
    //and "file.txt" the file we want to hash

    let args:Vec<String> = env::args().collect();

    
    if args.len() < 2 {//ensures we have 2 args in our vector programname and targetfile
        eprintln!("Error: Not enough arguments");
        eprintln!("Usage: cargo run -- <file_path>");
        return Err("This is NOT a crash, the program failed safely".into()); // returns our error to the main function
    }

    //we are creating a pointer to the content at args[1]
    //, not taking ownership just looking at the data
    let file_path = &args[1];

    //reading the file and storing the ascii characters 
    // the ? is like an if else statement. if fs::read successfully reads. ? will allow the data to be stored
    // if fs::read fails, it will return Err(the_error). The ? will see this and return it to our main function.
    let file_bytes = fs::read(file_path)?;

    //creates an empty mutable sha256 object 
    let mut hasher = Sha256::new();

    //hashes the bytes using SHA256 algorithm into an array of numbers
    hasher.update(&file_bytes);   

    //stores the array of numbers in an UN-mutable variable
    let hash_bytes = hasher.finalize();

    println!("\nFile:     {}", file_path);
    println!("\nRaw Bytes:  {:?}", hash_bytes);

    //at the moment hash_bytes holds an array of 32 numbers not a readable string
    //the hex::encode function will translate the 32 bytes 
    //into two character hexadecimal digits and output to a readable string
    println!("SHA256:   {}\n ",hex::encode(hash_bytes) );

    //returns to main showing the result of the program (successful)
    Ok(())


}