use std::error::Error;
use std::io;
use std::f32;
use std::process;
use serde::Deserialize;
use charts::{Chart, ScaleLinear, MarkerType, PointLabelPosition, Color, ScatterView};
use colored::*;

pub const INFINITY: f64 = f64::INFINITY;
pub const NEG_INFINITY: f64 = f64::NEG_INFINITY;

fn save_chart(data: &Vec<(f32, f32)>) {
	let width = 1500;
	let height = 1000;
	let (top, right, bottom, left) = (90, 40, 50, 60);
	
	let x = ScaleLinear::new()
		.set_domain(vec![0_f32, 275000_f32])
		.set_range(vec![0, width - left - right]);
	let y = ScaleLinear::new()
		.set_domain(vec![0_f32, 10000_f32])
		.set_range(vec![height - top - bottom, 0]);
	
	let scatter_view = ScatterView::new()
		.set_x_scale(&x)
		.set_y_scale(&y)
		.set_label_position(PointLabelPosition::E)
		.set_marker_type(MarkerType::Circle)
		.set_colors(Color::color_scheme_10())
		.load_data(&data).unwrap();
	
	Chart::new()
		.set_width(width)
		.set_height(height)
		.set_margins(top, right, bottom, left)
		.add_title(String::from("Linear Regression 42"))
		.add_view(&scatter_view)
		.add_axis_bottom(&x)
		.add_axis_left(&y)
		.add_left_axis_label("Prix")
		.add_bottom_axis_label("Kilometrage")
		.save("chart.svg").unwrap();
}

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

fn calculer_derive_partielle(records: &Vec<(f64, f64)>, old_theta_0: f64, old_theta_1: f64) -> (f64, f64){
	let m = records.len();
	let mut derive_theta_0 = 0.0;
	let mut derive_theta_1 = 0.0;
	
	
	for i in records {
		derive_theta_0 += (old_theta_0 + (old_theta_1 * i.0)) - i.1;
		derive_theta_1 += ((old_theta_0 + (old_theta_1 * i.0)) - i.1) * i.0;
	
	}
	//println!("{}: {}", "derive_theta_0".bright_cyan(), derive_theta_0);
	//println!("{}: {}", "derive_theta_1".bright_blue(), derive_theta_1);
	//println!("{}: {}", "old_theta_0".bright_cyan(), old_theta_0);
	//println!("{}: {}", "old_theta_1".bright_blue(), old_theta_1);
	derive_theta_0 = (1.0/m as f64) as f64 * derive_theta_0;
	derive_theta_1 = (1.0/m as f64) as f64 * derive_theta_1;
	//println!("{}: {}", "derive_theta_0".bright_cyan(), derive_theta_0);
	//println!("{}: {}", "derive_theta_1".bright_blue(), derive_theta_1);
	//println!();

	(derive_theta_0, derive_theta_1)
}

fn calculer_new_theta(learning_rate: f64, records: &Vec<(f64, f64)>, old_theta_0: f64, old_theta_1: f64) -> (f64, f64) {
	let (derive_theta_0, derive_theta_1) = calculer_derive_partielle(&records, old_theta_0, old_theta_0);
	let new_theta_0 = old_theta_0 - (learning_rate as f64 * derive_theta_0);
	let new_theta_1 = old_theta_1 - (learning_rate as f64 * derive_theta_1);

	(new_theta_0, new_theta_1)
}

fn main() {
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

	let mut max = INFINITY;
	let mut min = NEG_INFINITY;

	for el in &records {
		if el.0 > max {
			max = el.0;
		} else if el.0 < min {
			min = el.0;
		}
	}
	let scale = (max - min) as f64;

	let learning_rate = 0.1;
	let mut init_theta_0 = 0.0;
	let mut init_theta_1 = 0.0;
	let nb_iteration = 2000;
	let m = records.len();

	let mut tmp_0 = init_theta_0;
	let mut tmp_1 = init_theta_1;

	for _i in 1..nb_iteration+1 {
		let sum: (f64, f64) = (
			records.iter().fold(0.0, |acc, &val| {
				let scaled = (val.0 - min) as f64 / scale;
				acc + (init_theta_0 + init_theta_1 * scaled) - val.1 as f64
			}),
			records.iter().fold(0.0, |acc, &val| {
				let scaled = (val.0 - min) as f64 / scale;
				acc + ((init_theta_0 + init_theta_1 * scaled) - val.1 as f64) * scaled
			}),
		);

		// calculate slopes
		tmp_0 = learning_rate * sum.0 / m as f64;
		tmp_1 = learning_rate * sum.1 / m as f64;

		init_theta_0 -= tmp_0;
		init_theta_1 -= tmp_1;
	}

	init_theta_1 = init_theta_1 / scale;
	println!("theta_0: {}\ntheta_1: {}", init_theta_0, init_theta_1);
	
	//save_chart(&records);
}
