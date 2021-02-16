use std::process;
use serde::Deserialize;
use std::io;
use std::error::Error;
use colored::*;

#[derive(Debug, Deserialize)]
struct Record {
    km: f64,
    price: f64,
}

fn read_from_csv() -> Result<Vec<(f64, f64)>, Box<dyn Error>> {
	let mut rdr = csv::Reader::from_reader(io::stdin());
	let mut records: Vec<(f64, f64)> = Vec::new();
	for result in rdr.deserialize() {
		let record: Record = result?;
		records.push((record.km, record.price));
	}
	Ok(records)
}

fn main() {
	let mut max = f64::MIN;
    let mut min = f64::MAX;

    let records: Vec<(f64, f64)>;
	let csv = read_from_csv();
	match csv {
		Ok(v) => {
			records = v
		},
		Err(e) => {
			println!("error parsing: {:?}", e);
			process::exit(1);
		},
	}

	let len = records.len() as i64;
	for el in &records {
		if el.0 > max {
			max = el.0;
		} else if el.0 < min {
			min = el.0;
		}
	}
	let scale = (max - min) as f64;

    let rate = 0.01;
    let mut curr_theta: (f64, f64) = (0.0, 0.0);
    let mut tmp_theta: (f64, f64) = (1.0, 1.0);

    let mut i = 0;

    while tmp_theta.0.abs() > 0.0001 && tmp_theta.1.abs() > 0.0001 {
        let sum: (f64, f64) = (
            records.iter().fold(0.0, |acc, &val| {
                let scaled = (val.0 - min as f64) as f64 / scale;
                acc + (curr_theta.0 + curr_theta.1 * scaled) - val.1 as f64
            }),
            records.iter().fold(0.0, |acc, &val| {
                let scaled = (val.0 - min as f64) as f64 / scale;
                acc + ((curr_theta.0 + curr_theta.1 * scaled) - val.1 as f64) * scaled
            }),
        );

        // calculate slopes
        tmp_theta.0 = rate * sum.0 / len as f64;
        tmp_theta.1 = rate * sum.1 / len as f64;

        curr_theta.0 -= tmp_theta.0;
        curr_theta.1 -= tmp_theta.1;

        // show some stats
        if i == 0 {
            i = 100;
            println!("{}: {:?}", "curr".bright_cyan(), curr_theta);
            println!("{}: {:?}\n", "slope".bright_cyan(), tmp_theta);
        }

        i -= 1;
    }

    // scale back theta1
    curr_theta.1 = curr_theta.1 / scale;

	let final_theta = curr_theta;

    println!("{}: {:?}", "final_theta_0".bright_green(), final_theta.0);
	println!("{}: {:?}", "final_theta_1".bright_green(), final_theta.1);

}
