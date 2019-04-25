extern crate csv;

use std::num::ParseFloatError;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;


fn double_number(number_str: &str) -> Result<f64, ParseFloatError> {
    match number_str.parse::<f64>() {
        Ok(n) => Ok(n),
        Err(err) => Err(err),
    }
}

fn get_first_arg() -> Result<OsString, Box<Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn csv_to_f64(file_path:OsString) -> Vec<Vec<f64>>{
    let file = File::open(file_path).unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    let mut v:Vec<Vec<f64>> = Vec::new();
    for result in rdr.records() {
        let record = result.expect("a CSV record");
        let mut vec:Vec<f64> = Vec::new();
        for x in record.iter(){
            match double_number(x) {
                Ok(n) => vec.push(n),
                Err(_err) => vec.push(0.0),
            }
        }
        v.push(vec);
    }
    v
}

fn main() {
    println!("{:?}",csv_to_f64(get_first_arg().unwrap()));
}