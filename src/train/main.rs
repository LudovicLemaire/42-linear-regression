use std::process;
use serde::Deserialize;
use std::io;
use std::error::Error;
use colored::*;
mod draw_charts;
use draw_charts::{save_final_chart, save_line_chart};
use ini::Ini;

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
    let mut learning_curve: Vec<(f64, f64)> = Vec::new();
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

	let len = records.len() as f64;
	for el in &records {
		if el.0 > max {
			max = el.0;
		} else if el.0 < min {
			min = el.0;
		}
	}
	let scale: f64 = max - min;

    let rate = 0.1;
    let mut curr_theta_0: f64 = 0.0;
    let mut curr_theta_1: f64 = 0.0;
    let mut tmp_theta_0: f64 = 1.0;
    let mut tmp_theta_1: f64 = 1.0;

    let mut i = 0 as u8;
    let mut learning_iteration = 0 as u64;

    while tmp_theta_0.abs() > 0.0001 && tmp_theta_1.abs() > 0.0001 {
        let mut derive_theta_0: f64 = 0.0;
        let mut derive_theta_1: f64 = 0.0;
        for el in &records {
            let scaled: f64 = (el.0 - min) / scale;
            derive_theta_0 += (curr_theta_0 + curr_theta_1 * scaled) - el.1;
            derive_theta_1 += ((curr_theta_0 + curr_theta_1 * scaled) - el.1) * scaled;
        }

        tmp_theta_0 = rate * derive_theta_0 / len;
        tmp_theta_1 = rate * derive_theta_1 / len;

        curr_theta_0 -= tmp_theta_0;
        curr_theta_1 -= tmp_theta_1;


        match i {
            100 => {
                i = 0;
                // println!("{}: {}, {}", "curr".bright_cyan(), curr_theta_0, curr_theta_1 / scale); // could be used to show evolution of [ax+b]
                // println!("{}: {}, {}", "slope".bright_cyan(), tmp_theta_0, tmp_theta_1); // 
                learning_curve.push(( learning_iteration as f64 * 100.0, tmp_theta_0.abs()));
                learning_iteration += 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    // rescale theta1
    curr_theta_1 = curr_theta_1 / scale;

	let final_theta_0 = curr_theta_0;
    let final_theta_1 = curr_theta_1;

    println!("{}: {}", "final_theta_0".bright_green(), final_theta_0);
	println!("{}: {}", "final_theta_1".bright_green(), final_theta_1);

    save_final_chart(&records, final_theta_0, final_theta_1, "Prix".to_string(), "Kilométrage".to_string());
    save_line_chart(&learning_curve, "Learning".to_string(), "Iteration".to_string());

    let mut conf = Ini::new();
    conf.with_section(Some("thetas"))
        .set("theta_0", final_theta_0.to_string())
        .set("theta_1", final_theta_1.to_string());
    conf.write_to_file("data/theta.ini").unwrap();
}
