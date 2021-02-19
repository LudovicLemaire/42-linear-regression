use std::process;
use serde::Deserialize;
use std::io;
use std::error::Error;
use colored::*;
mod draw_charts;
use draw_charts::{save_final_chart};
use ini::Ini;
use draw_charts::DataMM;
#[derive(Debug, Deserialize)]
struct Record {
    km: f64,
    price: f64,
}

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

fn calculer_derive_partielle(records: &Vec<(f64, f64)>, old_theta_0: f64, old_theta_1: f64, _i: i32) -> (f64, f64) {
    let m = records.len();
    let mut der_theta_0 = 0.0;
    let mut der_theta_1 = 0.0;
    for curr_thetas in records {
        der_theta_0 += (old_theta_0 + (old_theta_1 * curr_thetas.0)) - curr_thetas.1;
        der_theta_1 += ((old_theta_0 + (old_theta_1 * curr_thetas.0)) - curr_thetas.1) * curr_thetas.0;
    }
    der_theta_0 = (1 as f64/m as f64) * der_theta_0;
    der_theta_1 = (1 as f64/m as f64) * der_theta_1;
    (der_theta_0, der_theta_1)
}

fn calculer_nv_theta(records: &Vec<(f64, f64)>, old_theta_0: f64, old_theta_1: f64, i: i32) -> (f64, f64) {
    let learning_rate = 0.1;
    let (der_theta_0, der_theta_1) = calculer_derive_partielle(&records, old_theta_0, old_theta_1, i);
    //println!("{}[{}]: {} {}", "der_theta".bright_blue(), i.to_string().bright_blue(), der_theta_0, der_theta_1);
    let nv_theta_0 = old_theta_0 - (learning_rate * der_theta_0);
    let nv_theta_1 = old_theta_1 - (learning_rate * der_theta_1);
    //println!("{}[{}]: {} {}", "nv_theta".bright_blue(), i.to_string().bright_blue(), nv_theta_0, nv_theta_1);
    (nv_theta_0, nv_theta_1)
}

fn normalize_elem(el: f64, min: f64, max: f64) -> f64 {
    (el - min) / (max - min)
}

fn denormalize_elem(el: f64, min: f64, max: f64) -> f64 {
    (el * (max - min)) + min
}

fn normalize_data(data: &Vec<(f64, f64)>, mm: &DataMM) -> Vec<(f64, f64)> {
    let mut new_data: Vec<(f64, f64)> = Vec::new();
    for el in data {
        new_data.push((normalize_elem(el.0, mm.min_0, mm.max_0), normalize_elem(el.1, mm.min_1, mm.max_1)));
    }
    new_data
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

    let mut mm = DataMM { min_0: f64::MAX, max_0: f64::MIN, min_1: f64::MAX, max_1: f64::MIN };
    for el in &records {
        if el.0 > mm.max_0 {
            mm.max_0 = el.0;
        }
        if el.0 < mm.min_0 {
            mm.min_0 = el.0;
        }
        if el.1 > mm.max_1 {
            mm.max_1 = el.1;
        }
        if el.1 < mm.min_1 {
            mm.min_1 = el.1;
        }
    }

    let new_records: Vec<(f64, f64)> = normalize_data(&records, &mm);

	let mut tmp_theta_0 = 0.0;
    let mut tmp_theta_1 = 0.0;
    let nb_iterartion = 20000;

    for i in 1..nb_iterartion {
        let (nv_theta_0, nv_theta_1) = calculer_nv_theta(&new_records, tmp_theta_0, tmp_theta_1, i);
        tmp_theta_0 = nv_theta_0;
        tmp_theta_1 = nv_theta_1;
    }

	let final_theta_0 = tmp_theta_0; //denormalize_elem(tmp_theta_0, min_0, max_0);
    let final_theta_1 = tmp_theta_1; //denormalize_elem(tmp_theta_1, min_1, max_1);

    println!("{}: {}", "final_theta_0".bright_green(), final_theta_0);
	println!("{}: {}", "final_theta_1".bright_green(), final_theta_1);

    println!("r: {}", denormalize_elem(final_theta_0 + final_theta_1 * normalize_elem(100 as f64, mm.min_0, mm.max_0), mm.min_1, mm.max_1));
    save_final_chart(&records, final_theta_0, final_theta_1, &category_names, &mm);

    let mut conf = Ini::new();
    conf.with_section(Some("thetas"))
        .set("theta_0", final_theta_0.to_string())
        .set("theta_1", final_theta_1.to_string());
	conf.with_section(Some("categories"))
        .set("x", category_names.0)
        .set("y", category_names.1);
    conf.write_to_file("data/theta.ini").unwrap();
}
