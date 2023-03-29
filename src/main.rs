use std::io::{BufReader, copy};
use std::fs::{File};
use std::env;
use std::time::Instant;
use flate2::write::GzEncoder;
use flate2::Compression;

fn main(){
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return;
    }

    let input = match File::open(&args[1]) {
        Ok(file) => file,
        Err(err) => panic!("Error opening input file: {}", err),
    };
    let output = match File::create(&args[2]) {
        Ok(file) => file,
        Err(err) => panic!("Error creating output file: {}", err),
    };
    
    let mut input = BufReader::new(input);
    let mut encoder = GzEncoder::new(output, Compression::default());

    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();

    let output = encoder.finish().unwrap();
    let input_len = input.get_ref().metadata().unwrap().len();
    let output_len = output.metadata().unwrap().len();

    println!("Source len: {:?}", input_len);
    println!("Target len: {:?}", output_len);
    println!("Elapsed: {:?}", start.elapsed());
}

