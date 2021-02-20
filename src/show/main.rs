use std::process;
use std::io;
use std::error::Error;
use colored::*;
mod draw_chart;
use draw_chart::{save_chart};

fn read_from_csv() -> Result<(Vec<(f64, f64)>, (String, String)), Box<dyn Error>> {
	let mut records: Vec<(f64, f64)> = Vec::new();
	let mut rdr = csv::Reader::from_reader(io::stdin());

    for result in rdr.records() {
        let record = result?;
		records.push((record[0].parse::<f64>().unwrap(), record[1].parse::<f64>().unwrap()));
    }
	let headers = rdr.headers()?;
    Ok((records, (headers[0].to_string(), headers[1].to_string())))
}

fn main() {
	let records: Vec<(f64, f64)>;
	let category_names: (String, String);
	let csv = read_from_csv();
	match csv {
		Ok(v) => {
			records = v.0;
			category_names = v.1;
		},
		Err(e) => {
			println!("error parsing: {:?}", e);
			process::exit(1);
		},
	}
    save_chart(&records, &category_names);
	println!("\nChart has been drawn {}", "successfully.".bright_green())
}
