use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use std::str::from_utf8;
use serde_json;
use serde_json::json;
use std::convert::TryInto;
use std::io::{self, Write};

fn demo<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}


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

pub fn poot() ->()
{
    println!("yikes");
}

pub fn passArray(list:  & Vec<u64>) -> bool
{
    let jsonData = json!({
        "array": list
    });
    println!("{:?}", list);
    let output = Command::new("python3")
    .arg("src/test2.py")
    .arg(jsonData.to_string())
    .output()
    .expect("Failed to execute command");

    // print response
    println!("status: {}", output.status);
io::stdout().write_all(&output.stdout).unwrap();
io::stderr().write_all(&output.stderr).unwrap();
    // println!("Response from python script: {}", from_utf8(output.stdout.as_slice()).expect("Failed to convert byte to string"));
    return false;
}

