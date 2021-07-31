use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use std::str::from_utf8;


pub fn test() -> ()
{
    println!("sanity check");
    let test_string = "Hello World!";
    // create python script process with test string as argument
    // note: output() waits for child to finish
    let output = Command::new("python3")
        .arg("src/test.py")
        .arg(test_string)
        .output()
        .expect("Failed to execute command");
    
    // print response
    println!("Response from python script: {}", from_utf8(output.stdout.as_slice()).expect("Failed to convert byte to string"));
}