use ini::Ini;
use std::process;
use std::io;
use std::io::Write;
use colored::*;

fn main() {
	let mut theta_0:f32 = 0.0;
	let mut theta_1:f32 = 0.0;

	let file = Ini::load_from_file("./data/theta.ini").unwrap();
	for (_sec, prop) in file.iter() {
		for (k, v) in prop.iter() {
			match k {
				"theta_0" => {
					theta_0 = v.parse::<f32>().unwrap_or_else(|e| {
						println!("{}: {}: {}", "error".red().bold(), k, e);
						process::exit(1);
					})
				}
				"theta_1" => {
					theta_1 = v.parse::<f32>().unwrap_or_else(|e| {
						println!("{}: {}: {}", "error".red().bold(), k, e);
						process::exit(1);
					})
				}
				_ => {
					println!("{}: {}", "error".red().bold(), "invalid key in .ini file");
					process::exit(1);
				}
			}
		}
	}

	if theta_0 == 0.0 && theta_1 == 0.0 {
		println!("{}: {}", "warning".yellow().bold(), "it looks like thetas aren't generated, did you forgot to train the model ?");
		process::exit(1);
	}

	println!("{}: {}", "theta_0".bright_purple(), theta_0);
	println!("{}: {}\n", "theta_1".bright_purple(), theta_1);

	let mut user_input = String::new();
	print!("{}: ", "Enter mileage".bright_cyan());
	io::stdout().flush().unwrap(); // force the print to show up

	io::stdin().read_line(&mut user_input).unwrap_or_else(|e| {
        println!("{}: {}", "error".red().bold(), e);
        process::exit(1);
    });
	
	let mileage = user_input.trim().parse::<f32>().unwrap_or_else(|e| {
		println!("{}: {}", "error".red().bold(), e);
		process::exit(1);
	});

	println!("{}: {}", "Expected price".bright_blue(), theta_0 + theta_1 * mileage);
}
