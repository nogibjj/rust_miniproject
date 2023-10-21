use csv::ReaderBuilder;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn main_with_args(_args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <input_file>", args[0]);
        return Ok(());
    }

    let input_file = &args[1];
    let output_file = "output.txt";

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

    let mut output = File::create(output_file)?;
    write!(output, "Average: {:.2}", average)?;

    println!("Average calculated and saved to {}", output_file);

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    main_with_args(args)
}
