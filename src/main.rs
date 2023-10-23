use std::error::Error;
use std::time::Instant;
use std::io::{Read};
use std::fs::File;
use csv::ReaderBuilder;
use std::process::Command;

pub fn main_with_args(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.len() != 2 {
        println!("Usage: {} <input_file>", args[0]);
        return Ok(());
    }

    let input_file = &args[1];

    let start_time = Instant::now();

    // Code for executing the main script
    let main_output = {
        let mut main_process = Command::new("./data_processing")
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to execute data_processing");

        let mut output = String::new();

        if let Some(ref mut stdout) = main_process.stdout {
            stdout.read_to_string(&mut output).expect("Failed to read stdout");
        }

        main_process.wait().expect("Failed to wait for data_processing to finish");

        output
    };

    println!("{}", main_output);

    let file = File::open(input_file)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut total = 0;
    let mut count = 0;

    for result in rdr.records() {
        let record = result?;
        if let Some(value) = record.get(1) {
            if let Ok(num) = value.parse::<i32>() {
                total += num;
                count += 1;
            }
        }
    }

    let average = if count > 0 {
        total as f32 / count as f32
    } else {
        0.0
    };

    let elapsed_time = start_time.elapsed();
    println!("Average calculated: {:.2}", average);
    println!("Time taken: {:?}", elapsed_time);

    // Get memory info
    if let Ok(output) = Command::new("free").arg("-m").output() {
        if let Ok(output_str) = String::from_utf8(output.stdout) {
            println!("{}", output_str);
        } else {
            println!("Failed to convert output to string");
        }
    } else {
        println!("Failed to execute command");
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    main_with_args(args)
}
