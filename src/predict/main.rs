use ini::Ini;
use std::process;
use std::io;
use std::io::Write;
use colored::*;
mod utils;
use utils::DataMM;
use utils::{normalize_elem, denormalize_elem, error_invalid_key};

fn main() {
	let mut theta_0:f64 = 0.0;
	let mut theta_1:f64 = 0.0;
	let mut label_x: Option<String> = None;
	let mut label_y: Option<String> = None;
	let mut mm = DataMM { min_0: 0.0, max_0: 0.0, min_1: 0.0, max_1: 0.0 };

	let file = Ini::load_from_file("./data/theta.ini").unwrap();
	for (s, prop) in file.iter() {
		let section = s.unwrap();
		for (k, v) in prop.iter() {
			match section {
				"thetas" => {
					match k {
						"theta_0" => {
							theta_0 = v.parse::<f64>().unwrap_or_else(|e| {
								println!("{}: {}: {}", "error".red().bold(), k, e);
								process::exit(1);
							})
						}
						"theta_1" => {
							theta_1 = v.parse::<f64>().unwrap_or_else(|e| {
								println!("{}: {}: {}", "error".red().bold(), k, e);
								process::exit(1);
							})
						}
						_ => {
							error_invalid_key(section, k);
						}
					}
				}
				"categories" => {
					match k {
						"x" => {
							label_x = Some(v.to_string());
						}
						"y" => {
							label_y = Some(v.to_string());
						}
						_ => {
							error_invalid_key(section, k);
						}
					}
				}
				"denormalize" => {
					match k {
						"min_0" => {
							mm.min_0 = v.parse::<f64>().unwrap_or_else(|e| {
								println!("{}: {}: {}", "error".red().bold(), k, e);
								process::exit(1);
							})
						}
						"max_0" => {
							mm.max_0 = v.parse::<f64>().unwrap_or_else(|e| {
								println!("{}: {}: {}", "error".red().bold(), k, e);
								process::exit(1);
							})
						}
						"min_1" => {
							mm.min_1 = v.parse::<f64>().unwrap_or_else(|e| {
								println!("{}: {}: {}", "error".red().bold(), k, e);
								process::exit(1);
							})
						}
						"max_1" => {
							mm.max_1 = v.parse::<f64>().unwrap_or_else(|e| {
								println!("{}: {}: {}", "error".red().bold(), k, e);
								process::exit(1);
							})
						}
						_ => {
							error_invalid_key(section, k);
						}
					}
				}
				_ => {
					println!("{}: invalid section [{}] in .ini file", "error".red().bold(), k);
					process::exit(1);
				}
			}
		}
	}

	if theta_0 == 0.0 && theta_1 == 0.0 {
		println!("{}: {}", "warning".yellow().bold(), "it looks like thetas aren't generated, did you forgot to train the model ?");
		process::exit(1);
	}

	if label_x.is_none() || label_y.is_none() {
		println!("{}: {}", "warning".yellow().bold(), "it looks like category names aren't generated, did you forgot to train the model ?");
		process::exit(1);
	}
	let label_x = label_x.unwrap();
	let label_y = label_y.unwrap();
	
	let mut user_input_category = String::new();
	println!("\nWhich category would you like to predict ?");

	println!("{} find {} from {}", "[0]:".bright_cyan(), label_y.bright_blue(), label_x.bright_blue());
	println!("{} find {} from {}\n", "[1]:".bright_cyan(), label_x.bright_blue(), label_y.bright_blue());
	print!("{}: ", "Enter 0 or 1".bright_cyan());
	io::stdout().flush().unwrap(); // force the print to show up

	io::stdin().read_line(&mut user_input_category).unwrap_or_else(|e| {
        println!("{}: {}", "error".red().bold(), e);
        process::exit(1);
    });
	
	let category_nb = user_input_category.trim().parse::<String>().unwrap_or_else(|e| {
		println!("{}: {}", "error".red().bold(), e);
		process::exit(1);
	});

	let category_needed: String;
	let category_predicted: String;
	let category_nb:&str = &category_nb;
	match category_nb {
		"0" => {
			category_needed = label_x;
			category_predicted = label_y;
		},
		"1" => {
			category_needed = label_y;
			category_predicted = label_x;
		},
		user_answer => {
			println!("{}: You must specify 0 or 1. You answered [{}]", "error".red().bold(), user_answer.bright_cyan());
			process::exit(1);
		}
	}

	let mut user_input_nb = String::new();
	print!("{} {}: ", "Enter".bright_cyan(), category_needed.bright_cyan());
	io::stdout().flush().unwrap(); // force the print to show up

	io::stdin().read_line(&mut user_input_nb).unwrap_or_else(|e| {
        println!("{}: {}", "error".red().bold(), e);
        process::exit(1);
    });
	
	let user_input_nb = user_input_nb.trim().parse::<f64>().unwrap_or_else(|e| {
		println!("{}: {}", "error".red().bold(), e);
		process::exit(1);
	});

	let prediction: f64;
	match category_nb {
		"0" => prediction = denormalize_elem(theta_0 + theta_1 * normalize_elem(user_input_nb as f64, mm.min_0, mm.max_0), mm.min_1, mm.max_1),
		_ => prediction = denormalize_elem((normalize_elem(user_input_nb as f64, mm.min_1, mm.max_1) - theta_0) / theta_1, mm.min_0, mm.max_0)
	}
	println!("{} {}: {}", "\nExpected".bright_green(), category_predicted.bright_green(), prediction);
}
