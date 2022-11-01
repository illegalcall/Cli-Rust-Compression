use std::io::{BufReader};
use std::io::copy;
use std::time::Instant;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;


fn main() {
    if args().len() !=3 {
        eprintln!("Usage: source target")
    }
    
    let mut input =BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let start = Instant::now();
    let mut encoder = GzEncoder::new(output, Compression::best());

    copy(&mut input, &mut encoder).unwrap();

    let output = encoder.finish().unwrap();

    println!("Source len: {:?}",input.get_ref().metadata().unwrap().len());
    println!("Target len: {:?}",output.metadata().unwrap().len());
    println!("Elapsed time: {:?}", start.elapsed());


}
