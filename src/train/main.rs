use std::process;
use std::io;
use std::error::Error;
use colored::*;
use ini::Ini;
mod draw_charts;
use draw_charts::{save_final_chart, save_line_chart, save_cost_chart, normalize_elem};
use draw_charts::DataMM;

fn read_from_csv() -> Result<(Vec<(f64, f64)>, (String, String)), Box<dyn Error>> {
	let mut records: Vec<(f64, f64)> = Vec::new();
	let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(io::stdin());

	for result in rdr.records() {
		let record = result?;
		records.push((
			record[0].parse::<f64>().unwrap_or_else(|e| {
				println!("{}: {}", "error".red().bold(), e);
				process::exit(1);
			}),
			record[1].parse::<f64>().unwrap_or_else(|e| {
				println!("{}: {}", "error".red().bold(), e);
				process::exit(1);
			})
		));
	}
	let headers = rdr.headers()?;
	Ok((records, (headers[0].to_string(), headers[1].to_string())))
}

fn calc_partial_derivative(records: &Vec<(f64, f64)>, old_theta_0: f64, old_theta_1: f64) -> (f64, f64) {
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

fn calc_new_theta(records: &Vec<(f64, f64)>, old_theta_0: f64, old_theta_1: f64, learning_rate: f64) -> (f64, f64, f64, f64) {
	let (der_theta_0, der_theta_1) = calc_partial_derivative(&records, old_theta_0, old_theta_1);
	let new_theta_0 = old_theta_0 - (learning_rate * der_theta_0);
	let new_theta_1 = old_theta_1 - (learning_rate * der_theta_1);
	(new_theta_0, new_theta_1, der_theta_0, der_theta_1)
}

fn mean_squared_error(records: &Vec<(f64, f64)>, new_theta_0: f64, new_theta_1: f64) -> f64 {
	let mut global_cost = 0.0;
	for el in records {
		let curr_cost = ((new_theta_0 + (new_theta_1 * el.0)) - el.1) * ((new_theta_0 + (new_theta_1 * el.0)) - el.1);
		global_cost += curr_cost;
	}
	(1.0/(2.0 * records.len() as f64)) * global_cost
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

	let normalized_records: Vec<(f64, f64)> = normalize_data(&records, &mm);

	let max_iteration = 20000;
	let precision = 0.00001;
	let learning_rate = 0.1;

	let mut tmp_theta_0 = 0.0;
	let mut tmp_theta_1 = 0.0;
	let mut slope_theta_0: f64 = 1.0;
	let mut slope_theta_1: f64 = 1.0;
	let mut curr_iteration = 0;
	let mut learning_curve: Vec<(f64, f64)> = Vec::new();
	let mut costs: Vec<f64> = Vec::new();

	while (curr_iteration < max_iteration) && (slope_theta_0.abs() > precision || slope_theta_1.abs() > precision) {
		let (new_theta_0, new_theta_1, cur_slope_theta_0, cur_slope_theta_1) = calc_new_theta(&normalized_records, tmp_theta_0, tmp_theta_1, learning_rate);
		slope_theta_0 = cur_slope_theta_0;
		slope_theta_1 = cur_slope_theta_1;
		costs.push(mean_squared_error(&normalized_records, new_theta_0, new_theta_1));
		tmp_theta_0 = new_theta_0;
		tmp_theta_1 = new_theta_1;
		curr_iteration += 1;

		if curr_iteration % 50 == 0 {
			learning_curve.push(( curr_iteration as f64, (cur_slope_theta_0.abs() + cur_slope_theta_1.abs())/2.0));
		}
	}

	let final_theta_0 = tmp_theta_0;
	let final_theta_1 = tmp_theta_1;

	println!("{}: [{}]", "Stopped at iteration".bright_yellow(), curr_iteration,);
	println!("{}: {}", "final_theta_0".bright_green(), final_theta_0);
	println!("{}: {}", "final_theta_1".bright_green(), final_theta_1);

	save_final_chart(&records, final_theta_0, final_theta_1, &category_names, &mm);
	let labels_curve = ("Iteration".to_string(), "Learning".to_string());
	save_line_chart(&learning_curve, &labels_curve);
	save_cost_chart(&costs);

	let mut conf = Ini::new();
	conf.with_section(Some("thetas"))
		.set("theta_0", final_theta_0.to_string())
		.set("theta_1", final_theta_1.to_string());
	conf.with_section(Some("categories"))
		.set("x", category_names.0)
		.set("y", category_names.1);
	conf.with_section(Some("denormalize"))
		.set("min_0", mm.min_0.to_string())
		.set("max_0", mm.max_0.to_string())
		.set("min_1", mm.min_1.to_string())
		.set("max_1", mm.max_1.to_string());
	conf.write_to_file("data/theta.ini").unwrap();
}
